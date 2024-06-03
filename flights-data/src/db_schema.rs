use chrono::{DateTime, Utc};
use sea_query::Iden;
use std::fmt;

#[derive(Iden)]
pub enum Locations {
    Table,
    All,
    Id,
    Name,
    ContinentId,
    RegionId,
    CountryId,
    SubdivisionId,
    AutonomousId,
    CityId,
    LocationType,
}

pub enum LocationTypeEnum {
    Airport,
    AutonomousTerritory,
    City,
    Country,
    Continent,
    Subdivision,
    Region,
}

impl fmt::Display for LocationTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LocationTypeEnum::Airport => "airport",
            LocationTypeEnum::Country => "country",
            LocationTypeEnum::AutonomousTerritory => "autonomous",
            LocationTypeEnum::City => "city",
            LocationTypeEnum::Continent => "continent",
            LocationTypeEnum::Region => "region",
            LocationTypeEnum::Subdivision => "subdivision",
        };
        write!(f, "{}", s)
    }
}

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
