use std::env;
use chrono::{Datelike, Days, Utc, Weekday};
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::api_client::{FlightsQuery, search_flights};
use crate::models::{NewTrip, Route};
use std::result::Result;

mod api_client;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::routes::dsl::*;

    dotenv().ok();
    load_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("KIWI_API_KEY").expect("KIWI_API_KEY must be set");

    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let db_routes: Vec<Route> = routes.select(Route::as_select()).load(&mut conn).expect("error fetching routes");
    println!("{:?}", db_routes);

    let date_to = Utc::now().checked_add_days(Days::new(90)).expect("");

    for route in db_routes {
        let flights_query = FlightsQuery {
            fly_from: route.airport_code.clone(),
            fly_to: route.country_code.clone(),
            date_from: Utc::now(),
            date_to: date_to.clone(),
            nights_in_dst_from: 2,
            nights_in_dst_to: 7,
            max_stopovers: 0,
        };
        let possible_flights = search_flights(api_key.clone(), flights_query).await?;
        println!("found {} flights for {}-{}", possible_flights.results, route.airport_code, route.country_code);

        let trips: Vec<NewTrip> = possible_flights.data.into_iter()
            .filter_map(|trip| {
                if !is_week_long_trip(&trip) && !is_weekend_getaway(&trip) {
                    return None;
                }
                let trip_type = if is_weekend_getaway(&trip) { 1 } else { 2 };
                return Some(NewTrip {
                    airport_code: trip.fly_from.clone(),
                    country_code: trip.country_to.code.clone(),
                    city_code: trip.city_code_to.clone(),
                    city_name: trip.city_to.clone(),
                    depart_at: trip.utc_departure().date_naive(),
                    arrive_at: trip.utc_return().date_naive(),
                    price: trip.price as i16,
                    airline: "placeholder-for-now".to_string(),
                    trip_type,
                    inserted_at: Utc::now().date_naive(),
                });
            }).collect();

        diesel::insert_into(schema::trips::table)
            .values(&trips)
            .execute(&mut conn)
            .expect("");
    }

    return Ok(());
}

fn is_week_long_trip(trip: &api_client::Trip) -> bool {
    return trip.length_in_nights >= 4;
}

fn is_weekend_getaway(trip: &api_client::Trip) -> bool {
    return trip.length_in_nights <= 3 &&
        (trip.utc_departure().weekday() == Weekday::Fri || trip.utc_departure().weekday() == Weekday::Sat) &&
        (trip.utc_return().weekday() == Weekday::Sun || trip.utc_return().weekday() == Weekday::Mon);
}

fn load_env() {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

