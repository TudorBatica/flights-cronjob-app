use sea_query::Iden;

#[derive(Iden)]
pub enum MonitoredRoutes {
    Table,
    MonitoredBy,
    AirportCode,
    CountryCode,
    Budget,
    TripType,
}

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
