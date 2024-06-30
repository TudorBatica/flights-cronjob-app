use flights_data::db_schema::LocationTypeEnum;
use serde::Deserialize;

#[derive(Deserialize)]
struct LocationsQuery {
    ltype: LocationTypeEnum,
    term: Option<String>,
}
