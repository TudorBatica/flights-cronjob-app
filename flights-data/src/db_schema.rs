use sea_query::Iden;

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
