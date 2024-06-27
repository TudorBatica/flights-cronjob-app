use chrono::{DateTime, Days, Duration, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres, Row};

use flights_data::db_schema::Itineraries::ItineraryType;
use flights_data::db_schema::{Flights, Itineraries, Route, Routes};

use crate::api_client;
use crate::api_client::{FlightsQuery, Trip};
use crate::configuration::Settings;

pub async fn run(config: &Settings, pool: &Pool<Postgres>) {
    let routes = fetch_routes_to_scan(config, pool).await;
    let now = Utc::now();
    for route in &routes {
        println!(
            "Finding new itineraries for {}-{}",
            route.from_location_id, route.to_location_id
        );
        store_new_itineraries(route, config, pool).await;
    }

    println!("Deleting outdated itineraries");
    delete_itineraries_before(now, pool).await;

    println!("Updating last scan date for all updated routes");
    update_routes_last_scan_date(routes, now, pool).await;
}

async fn fetch_routes_to_scan(config: &Settings, pool: &Pool<Postgres>) -> Vec<Route> {
    let datetime = Utc::now() - Duration::hours(config.hours_between_scans);
    let query = Query::select()
        .columns([
            Routes::FromLocationId,
            Routes::ToLocationId,
            Routes::LastScan,
        ])
        .from(Routes::Table)
        .and_where(
            Expr::col(Routes::LastScan)
                .is_null()
                .or(Expr::col(Routes::LastScan).lt(datetime.to_rfc3339())),
        )
        .to_string(PostgresQueryBuilder);

    return sqlx::query_as(&query).fetch_all(pool).await.unwrap();
}

async fn store_new_itineraries(route: &Route, config: &Settings, pool: &Pool<Postgres>) {
    let response = api_client::search_flights(
        &config.kiwi_api_key,
        &FlightsQuery {
            fly_from: route.from_location_id.clone(),
            fly_to: route.to_location_id.clone(),
            date_from: Utc::now(),
            date_to: Utc::now().checked_add_days(Days::new(160)).unwrap(),
            nights_in_dst_from: 2,
            nights_in_dst_to: 14,
            max_stopovers: 1,
        },
    )
    .await;
    if let Err(err) = response {
        println!(
            "Could not fetch flights from KIWI for route {:?} {:?}",
            route, err
        );
        return;
    }
    let response = response.unwrap();
    for itinerary in response.data {
        let itinerary_id = store_itinerary(&itinerary, pool).await;
        store_flights(itinerary, pool, itinerary_id).await;
    }
}

async fn store_itinerary(trip: &Trip, pool: &Pool<Postgres>) -> i32 {
    let departure_flight = trip
        .route
        .iter()
        .min_by(|x, y| x.utc_departure.cmp(&y.utc_departure))
        .unwrap();
    let return_flight = trip
        .route
        .iter()
        .max_by(|x, y| x.utc_departure.cmp(&y.utc_departure))
        .unwrap();
    let insert_itinerary_query = Query::insert()
        .into_table(Itineraries::Table)
        .columns([
            Itineraries::FromAirportId,
            Itineraries::ToAirportId,
            Itineraries::Price,
            Itineraries::BookingLink,
            Itineraries::DepartureDepartAtUtc,
            Itineraries::DepartureArriveAtUtc,
            Itineraries::ReturnDepartAtUtc,
            Itineraries::ReturnArriveAtUtc,
            Itineraries::Stopovers,
            Itineraries::InsertedAt,
            ItineraryType,
        ])
        .values_panic([
            trip.fly_from.clone().into(),
            trip.fly_to.clone().into(),
            (trip.price as u16).into(),
            trip.deep_link.clone().into(),
            departure_flight.utc_departure.to_rfc3339().into(),
            departure_flight.utc_arrival.to_rfc3339().into(),
            return_flight.utc_departure.to_rfc3339().into(),
            return_flight.utc_arrival.to_rfc3339().into(),
            (trip.route.len() - 2).to_string().into(),
            Utc::now().to_rfc3339().into(),
            "weekly".into(),
        ])
        .returning_col(Itineraries::Id)
        .to_string(PostgresQueryBuilder);

    return sqlx::query(&insert_itinerary_query)
        .fetch_one(pool)
        .await
        .unwrap()
        .try_get("id")
        .unwrap();
}

async fn store_flights(trip: Trip, pool: &Pool<Postgres>, itinerary_id: i32) {
    let insert_query = {
        let mut insert_statement = Query::insert()
            .into_table(Flights::Table)
            .columns([
                Flights::ItineraryId,
                Flights::FromAirportId,
                Flights::ToAirportId,
                Flights::DepartAtUtc,
                Flights::ArriveAtUtc,
                Flights::Airline,
                Flights::FlightNumber,
            ])
            .to_owned();

        for flight in trip.route {
            insert_statement
                .values_panic([
                    itinerary_id.into(),
                    flight.fly_from.into(),
                    flight.fly_to.into(),
                    flight.utc_departure.to_rfc3339().into(),
                    flight.utc_arrival.to_rfc3339().into(),
                    flight.airline.into(),
                    flight.flight_no.into(),
                ])
                .to_string(PostgresQueryBuilder);
        }

        insert_statement.to_string(PostgresQueryBuilder)
    };

    let _ = sqlx::query(&insert_query).execute(pool).await;
}

async fn delete_itineraries_before(date_time: DateTime<Utc>, pool: &Pool<Postgres>) {
    let delete_statement = Query::delete()
        .from_table(Itineraries::Table)
        .and_where(Expr::col(Itineraries::InsertedAt).lt(date_time.to_rfc3339()))
        .to_string(PostgresQueryBuilder);
    let _ = sqlx::query(&delete_statement).execute(pool).await;
}

async fn update_routes_last_scan_date(
    routes: Vec<Route>,
    date_time: DateTime<Utc>,
    pool: &Pool<Postgres>,
) {
    let routes: Vec<(&String, &String)> = routes
        .iter()
        .map(|r| (&r.from_location_id, &r.to_location_id))
        .collect();
    let update_statement = Query::update()
        .table(Routes::Table)
        .values([(Routes::LastScan, date_time.to_rfc3339().into())])
        .and_where(
            Expr::tuple([
                Expr::col(Routes::FromLocationId).into(),
                Expr::col(Routes::ToLocationId).into(),
            ])
            .in_tuples(routes),
        )
        .to_string(PostgresQueryBuilder);
    let _ = sqlx::query(&update_statement).execute(pool).await;
}
