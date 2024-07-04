use actix_web::{post, web, HttpResponse, Responder};
use sea_query::{Expr, Func, OnConflict, PostgresQueryBuilder, Query};
use sqlx::PgPool;

use flights_data::db_schema::{
    DistanceKmFunction, LocationTypeEnum, Locations, MonitoredTrips, Routes, UserRoutes,
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
    insert_missing_user_routes(&departure_airports, &destination_airports, &pool).await;
    insert_monitored_trip(&pool).await;

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

async fn insert_missing_user_routes(
    departure_airports: &Vec<String>,
    destination_airports: &Vec<String>,
    pool: &web::Data<PgPool>,
) {
    let query = {
        let mut statement = Query::insert()
            .into_table(UserRoutes::Table)
            .columns([
                UserRoutes::UserId,
                UserRoutes::FromLocationId,
                UserRoutes::ToLocationId,
            ])
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .to_owned();
        for departure in departure_airports {
            for destination in destination_airports {
                statement.values_panic([1.into(), departure.into(), destination.into()]);
            }
        }
        statement.to_string(PostgresQueryBuilder)
    };
    sqlx::query(&query).execute(pool.get_ref()).await.unwrap();
}

async fn insert_monitored_trip(pool: &web::Data<PgPool>) {
    let query = Query::insert()
        .into_table(MonitoredTrips::Table)
        .columns([MonitoredTrips::UserId, MonitoredTrips::Name])
        .values_panic([1.into(), "My awesome trip".into()])
        .to_string(PostgresQueryBuilder);

    sqlx::query(&query).execute(pool.get_ref()).await.unwrap();
}
