use std::env;
use std::error::Error;
use std::result::Result;

use chrono::{Datelike, DateTime, Days, NaiveDateTime, Utc, Weekday};
use diesel::delete;
use diesel::prelude::*;
use dotenvy::dotenv;

use schema::routes::dsl::*;
use schema::trips::dsl::*;

use crate::api_client::{FlightsQuery, search_flights};
use crate::models::{NewTrip, Route};

mod api_client;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("KIWI_API_KEY").expect("KIWI_API_KEY must be set");
    let monitored_period: u64 = env::var("MONITORED_PERIOD_LENGTH_DAYS")
        .expect("MONITORED_PERIOD must be set")
        .parse()
        .expect("MONITORED_PERIOD var not a valid number");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let db_routes: Vec<Route> = routes.select(Route::as_select()).load(&mut conn).expect("error fetching routes");
    let date_to = Utc::now().checked_add_days(Days::new(monitored_period)).unwrap();
    let now = Utc::now().naive_utc();

    for route in db_routes {
        insert_new_trips(&api_key, &mut conn, date_to, now, route).await?;
    }

    delete_old_trips(&mut conn, now);
    return Ok(());
}

async fn insert_new_trips(api_key: &str, conn: &mut PgConnection,
                          date_to: DateTime<Utc>, now: NaiveDateTime,
                          route: Route) -> Result<(), Box<dyn Error>> {
    let flights_query = FlightsQuery {
        fly_from: route.airport_code.clone(),
        fly_to: route.country_code.clone(),
        date_from: Utc::now(),
        date_to: date_to.clone(),
        nights_in_dst_from: 2,
        nights_in_dst_to: 7,
        max_stopovers: 0,
    };
    let possible_flights = search_flights(api_key, flights_query).await?;
    let new_trips: Vec<NewTrip> = possible_flights.data.into_iter()
        .filter(|trip| is_week_long_trip(trip) || is_weekend_getaway(trip))
        .map(|trip| {
            let trip_type_value = if is_weekend_getaway(&trip) { 1 } else { 2 };
            return NewTrip {
                airport_code: trip.fly_from.clone(),
                country_code: trip.country_to.code.clone(),
                city_code: trip.city_code_to.clone(),
                city_name: trip.city_to.clone(),
                depart_at: trip.utc_departure().date_naive(),
                arrive_at: trip.utc_return().date_naive(),
                price: trip.price as i16,
                airline: "placeholder-for-now".to_string(),
                trip_type: trip_type_value,
                inserted_at: now.clone(),
            };
        }).collect();

    diesel::insert_into(schema::trips::table)
        .values(&new_trips)
        .execute(conn)
        .expect("");
    Ok(())
}

fn delete_old_trips(conn: &mut PgConnection, date_before: NaiveDateTime) {
    let old_trips = trips.filter(inserted_at.lt(date_before));
    delete(old_trips).execute(conn).unwrap();
}

fn is_week_long_trip(trip: &api_client::Trip) -> bool {
    return trip.length_in_nights >= 4;
}

fn is_weekend_getaway(trip: &api_client::Trip) -> bool {
    return trip.length_in_nights <= 3 &&
        (trip.utc_departure().weekday() == Weekday::Fri || trip.utc_departure().weekday() == Weekday::Sat) &&
        (trip.utc_return().weekday() == Weekday::Sun || trip.utc_return().weekday() == Weekday::Mon);
}
