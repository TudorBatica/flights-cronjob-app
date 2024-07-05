use actix_web::{post, web, HttpResponse, Responder};
use sea_query::{Expr, Func, OnConflict, PostgresQueryBuilder, Query};
use sqlx::PgPool;

use flights_data::db_schema::{
    DistanceKmFunction, LocationTypeEnum, Locations, MonitoredTrips, Routes, TripRoutes,
};

#[derive(Debug, serde::Deserialize)]
pub struct NewTripRequest {
    destination_type: String,
    departure_city_id: String,
    departure_radius: u16,
    destination_city_id: Option<String>,
    destination_city_radius: Option<u16>,
    destination_country_id: Option<String>,
    destination_country_subdivisions: Vec<String>,
}

#[derive(sqlx::FromRow, Debug)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

#[post("/api/register_trip")]
pub async fn register(
    request_data: web::Json<NewTripRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    println!("Received new monitor request {:?}", request_data);

    let departure_airports = get_airports_in_range(
        request_data.departure_city_id.as_str(),
        request_data.departure_radius,
        &pool,
    )
    .await;

    let destination_airports = if request_data.destination_type == "city" {
        get_airports_in_range(
            request_data.destination_city_id.clone().unwrap().as_str(),
            request_data.destination_city_radius.unwrap(),
            &pool,
        )
        .await
    } else {
        get_airports_for(
            request_data.destination_country_id.clone().unwrap(),
            request_data.destination_country_subdivisions.clone(),
            &pool,
        )
        .await
    };

    println!(
        "will add routes {:?} {:?}, a total of {}",
        departure_airports,
        destination_airports,
        departure_airports.len() * destination_airports.len()
    );

    insert_missing_routes(&departure_airports, &destination_airports, &pool).await;
    let trip_name = get_trip_name(&request_data, &pool).await;
    let trip_id = insert_monitored_trip(trip_name, &pool).await;
    insert_trip_routes(&departure_airports, &destination_airports, trip_id, &pool).await;

    return HttpResponse::Ok();
}

async fn get_airports_in_range(
    city_id: &str,
    radius: u16,
    pool: &web::Data<PgPool>,
) -> Vec<String> {
    let city_query = Query::select()
        .from(Locations::Table)
        .columns([Locations::Latitude, Locations::Longitude])
        .and_where(Expr::col(Locations::Id).eq(city_id))
        .to_string(PostgresQueryBuilder);

    let city_coordinates: Coordinates = sqlx::query_as(&city_query)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    let distance_func = Func::cust(DistanceKmFunction).args([
        Expr::col(Locations::Latitude).into(),
        Expr::col(Locations::Longitude).into(),
        city_coordinates.latitude.into(),
        city_coordinates.longitude.into(),
    ]);

    let airports_query = Query::select()
        .from(Locations::Table)
        .columns(vec![Locations::Id])
        .and_where(Expr::col(Locations::LocationType).eq(LocationTypeEnum::Airport.to_string()))
        .and_where(
            Expr::col(Locations::CityId)
                .eq(city_id)
                .or(Expr::expr(distance_func).lte(radius)),
        )
        .to_string(PostgresQueryBuilder);

    let airports: Vec<String> = sqlx::query_scalar(&airports_query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    return airports;
}

async fn get_airports_for(
    country_id: String,
    subdivisions: Vec<String>,
    pool: &web::Data<PgPool>,
) -> Vec<String> {
    let query = if subdivisions.is_empty() {
        Query::select()
            .columns(vec![Locations::Id])
            .from(Locations::Table)
            .and_where(Expr::col(Locations::LocationType).eq(LocationTypeEnum::Airport.to_string()))
            .and_where(Expr::col(Locations::CountryId).eq(country_id))
            .to_string(PostgresQueryBuilder)
    } else {
        Query::select()
            .columns(vec![Locations::Id])
            .from(Locations::Table)
            .and_where(Expr::col(Locations::LocationType).eq(LocationTypeEnum::Airport.to_string()))
            .and_where(Expr::col(Locations::SubdivisionId).is_in(subdivisions))
            .to_string(PostgresQueryBuilder)
    };

    let airports: Vec<String> = sqlx::query_scalar(&query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    return airports;
}

async fn insert_missing_routes(
    departure_airports: &Vec<String>,
    destination_airports: &Vec<String>,
    pool: &web::Data<PgPool>,
) {
    let query = {
        let mut statement = Query::insert()
            .into_table(Routes::Table)
            .columns([Routes::FromLocationId, Routes::ToLocationId])
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .to_owned();
        for departure in departure_airports {
            for destination in destination_airports {
                statement.values_panic([departure.into(), destination.into()]);
            }
        }
        statement.to_string(PostgresQueryBuilder)
    };
    sqlx::query(&query).execute(pool.get_ref()).await.unwrap();
}

async fn insert_trip_routes(
    departure_airports: &Vec<String>,
    destination_airports: &Vec<String>,
    trip_id: i32,
    pool: &web::Data<PgPool>,
) {
    let query = {
        let mut statement = Query::insert()
            .into_table(TripRoutes::Table)
            .columns([
                TripRoutes::MonitoredTripId,
                TripRoutes::FromAirportId,
                TripRoutes::ToAirportId,
            ])
            .to_owned();
        for departure in departure_airports {
            for destination in destination_airports {
                statement.values_panic([trip_id.into(), departure.into(), destination.into()]);
            }
        }
        statement.to_string(PostgresQueryBuilder)
    };
    sqlx::query(&query).execute(pool.get_ref()).await.unwrap();
}

async fn insert_monitored_trip(name: String, pool: &web::Data<PgPool>) -> i32 {
    let query = Query::insert()
        .into_table(MonitoredTrips::Table)
        .columns([MonitoredTrips::UserId, MonitoredTrips::Name])
        .values_panic([1.into(), name.into()])
        .returning(Query::returning().columns([MonitoredTrips::Id]))
        .to_string(PostgresQueryBuilder);

    return sqlx::query_scalar(&query)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
}

async fn get_trip_name(trip_request: &NewTripRequest, pool: &web::Data<PgPool>) -> String {
    let departure_city_query = Query::select()
        .from(Locations::Table)
        .columns([Locations::Name])
        .and_where(Expr::col(Locations::Id).eq(trip_request.departure_city_id.as_str()))
        .to_string(PostgresQueryBuilder);
    let departure_city: String = sqlx::query_scalar(&departure_city_query)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    let destination_id = trip_request
        .destination_city_id
        .clone()
        .unwrap_or_else(|| trip_request.destination_country_id.clone().unwrap());
    let destination_options = if trip_request.destination_city_radius.is_some() {
        format!(
            "(+{}km)",
            trip_request.destination_city_radius.clone().unwrap()
        )
    } else {
        format!(
            "({} regions)",
            trip_request.destination_country_subdivisions.len()
        )
    };

    let destination_query = Query::select()
        .from(Locations::Table)
        .columns([Locations::Name])
        .and_where(Expr::col(Locations::Id).eq(destination_id))
        .to_string(PostgresQueryBuilder);
    let destination: String = sqlx::query_scalar(&destination_query)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    return format!(
        "{dep}(+{dep_radius}km) to {dest}{dest_options}",
        dep = departure_city,
        dep_radius = trip_request.departure_radius,
        dest = destination,
        dest_options = destination_options
    );
}
