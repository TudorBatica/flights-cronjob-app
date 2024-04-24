use std::error::Error;
use std::result::Result;

use chrono::{Datelike, DateTime, Days, Utc, Weekday};
use sea_query::{Cond, Expr, Iden, PostgresQueryBuilder, Query};
use sqlx::PgPool;

use crate::api_client::{FlightsQuery, Trip};

mod api_client;
mod configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = configuration::get_configuration();
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    let start_time = Utc::now();

    let monitored_routes = sqlx::query!("SELECT DISTINCT airport_code, country_code FROM monitored_routes;")
        .fetch_all(&pool)
        .await
        .unwrap();

    let date_to = Utc::now()
        .checked_add_days(Days::new(configuration.monitored_period_length_days as u64))
        .unwrap();

    let flights_queries: Vec<FlightsQuery> = monitored_routes.iter()
        .map(|route| {
            FlightsQuery {
                fly_from: route.airport_code.clone(),
                fly_to: route.country_code.clone(),
                date_from: Utc::now(),
                date_to,
                nights_in_dst_from: 2,
                nights_in_dst_to: 7,
                max_stopovers: 0,
            }
        }).collect();


    for query in flights_queries {
        let new_trips = api_client::search_flights(&configuration.kiwi_api_key, &query).await;
        match new_trips {
            Ok(trips) => {
                println!("Retrieved {} round-flights", trips.results);
                let eligible_trips = trips.data.into_iter()
                    .filter(|trip| is_week_long_trip(trip) || is_weekend_getaway(trip))
                    .collect();
                insert_trips(&pool, eligible_trips).await
            }
            Err(e) => {
                println!("Could not retrieve flights for query {:?}: {:?}", &query, e)
            }
        }
    }

    delete_trips_inserted_before(&pool, start_time).await;
    return Ok(());
}

//todo: move to flights-data
#[derive(Iden)]
pub enum Trips {
    Table,
    TripId,
    AirportCode,
    CountryCode,
    DepartAt,
    ReturnAt,
    Price,
    TripType,
    InsertedAt,
    CityCode,
    CityName,
}

async fn insert_trips(pool: &PgPool, items: Vec<Trip>) {
    let mut query = Query::insert()
        .into_table(Trips::Table)
        .columns(vec![
            Trips::AirportCode,
            Trips::CountryCode,
            Trips::DepartAt,
            Trips::ReturnAt,
            Trips::Price,
            Trips::TripType,
            Trips::InsertedAt,
            Trips::CityCode,
            Trips::CityName,
        ]).to_owned();

    for trip in items {
        query.values_panic(vec![
            trip.fly_from.clone().into(),
            trip.country_to.code.clone().into(),
            trip.utc_departure().to_rfc3339().into(),
            trip.utc_return().to_rfc3339().into(),
            trip.price.into(),
            get_trip_type(&trip).into(),
            Utc::now().to_rfc3339().into(),
            trip.city_code_to.into(),
            trip.city_to.into(),
        ]);
    }
    let query = query.to_string(PostgresQueryBuilder);

    match sqlx::query(&query).execute(pool).await {
        Ok(_) => {}
        Err(e) => println!("Encountered error {:?} when executing {}", e, query)
    };
}


async fn delete_trips_inserted_before(pool: &PgPool, date_time: DateTime<Utc>) {
    let query = Query::delete()
        .from_table(Trips::Table)
        .cond_where(Cond::all().add(Expr::col(Trips::InsertedAt).lt(date_time.to_rfc3339())))
        .to_string(PostgresQueryBuilder);

    match sqlx::query(&query).execute(pool).await {
        Ok(_) => {}
        Err(e) => println!("Encountered error {:?} when executing {}", e, query)
    }
}

fn get_trip_type(trip: &Trip) -> u8 {
    if is_weekend_getaway(trip) { 1 } else { 2 }
}

fn is_week_long_trip(trip: &Trip) -> bool {
    trip.length_in_nights >= 4
}

fn is_weekend_getaway(trip: &Trip) -> bool {
    trip.length_in_nights <= 3
        && (trip.utc_departure().weekday() == Weekday::Fri
        || trip.utc_departure().weekday() == Weekday::Sat)
        && (trip.utc_return().weekday() == Weekday::Sun
        || trip.utc_return().weekday() == Weekday::Mon)
}
