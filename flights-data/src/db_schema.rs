use chrono::{DateTime, Utc};
use sea_query::Iden;

#[derive(Iden)]
pub enum MonitoredRoutes {
    Table,
    All,
    MonitoredBy,
    AirportCode,
    CountryCode,
    Budget,
    TripType,
}

#[derive(Iden)]
pub enum Trips {
    Table,
    All,
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

#[derive(sqlx::FromRow, Debug)]
pub struct Trip {
    pub airport_code: String,
    pub country_code: String,
    pub price: i16,
    pub depart_at: DateTime<Utc>,
    pub return_at: DateTime<Utc>,
    pub trip_type: i16,
    pub inserted_at: DateTime<Utc>,
    pub city_code: String,
    pub city_name: String,
}
