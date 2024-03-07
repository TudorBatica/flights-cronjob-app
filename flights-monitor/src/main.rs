use std::env;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::api_client::{FlightsQuery, search_flights};
use crate::models::{NewUser, User};

mod api_client;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();

    insert_user(connection, "tudor@gmail.com", "Tudorescu");
    insert_user(connection, "tb@gmail.com", "tbescu");

    let db_users = users.select(User::as_select()).load(connection).expect("error fetching users");
    println!("{:?}", db_users);

    // let query = FlightsQuery {
    //     fly_from: "OTP".to_string(),
    //     fly_to: "GR".to_string(),
    //     date_from: d("01/03/2024"),
    //     date_to: d("01/05/2024"),
    //     budget: 100,
    //     nights_in_dst_from: 2,
    //     nights_in_dst_to: 7,
    //     max_stopovers: 0,
    // };
    // println!("{:?}", query);
    //
    // let flights = search_flights(query).await?;
    // println!("{:?}", flights);

    return Ok(());
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

pub fn insert_user(conn: &mut PgConnection, email: &str, name: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { email, name };

    return diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user");
}

fn d(s: &str) -> DateTime<Utc> {
    let date = NaiveDate::parse_from_str(s, "%d/%m/%Y").unwrap();
    return DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc);
}