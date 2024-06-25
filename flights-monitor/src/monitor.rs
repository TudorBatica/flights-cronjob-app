use chrono::{Days, Duration, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres, Row};

use flights_data::db_schema::Itineraries::ItineraryType;
use flights_data::db_schema::{Itineraries, Route, Routes};

use crate::api_client;
use crate::api_client::{FlightsQuery, Trip};
use crate::configuration::Settings;

pub async fn run(config: &Settings, pool: &Pool<Postgres>) {
    let routes = fetch_routes_to_scan(config, pool).await;
    for route in routes {
        store_new_itineraries(route, config, pool).await;
    }
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

async fn store_new_itineraries(route: Route, config: &Settings, pool: &Pool<Postgres>) {
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
    match response {
        Ok(response) => {
            for trip in response.data {
                store_itinerary_and_flights(trip, pool).await;
            }
        }
        Err(err) => {
            println!(
                "Could not fetch flights from KIWI for route {:?} {:?}",
                route, err
            );
            return;
        }
    }
}

async fn store_itinerary_and_flights(trip: Trip, pool: &Pool<Postgres>) {
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
            trip.fly_from.into(),
            trip.fly_to.into(),
            (trip.price as u16).into(),
            trip.deep_link.into(),
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

    let itinerary_id: i32 = sqlx::query(&insert_itinerary_query)
        .fetch_one(pool)
        .await
        .unwrap()
        .try_get("id")
        .unwrap();
    // let insert_flights_query = Query::insert()
    //     .into_table(Fli)
}
