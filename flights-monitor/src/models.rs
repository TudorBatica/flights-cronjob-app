use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::airports)]
#[diesel(primary_key(code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Airport {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::countries)]
#[diesel(primary_key(code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::routes)]
#[diesel(primary_key(airport_code, country_code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Route {
    pub airport_code: String,
    pub country_code: String,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = crate::schema::user_routes)]
pub struct UserRoute {
    pub user_id: i32,
    pub airport_code: String,
    pub country_code: String,
    pub budget: i16,
    pub trip_type: i16,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::trips)]
#[diesel(primary_key(trip_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trip {
    pub trip_id: i32,
    pub airport_code: String,
    pub country_code: String,
    pub city_code: String,
    pub city_name: String,
    pub depart_at: chrono::NaiveDate,
    pub arrive_at: chrono::NaiveDate,
    pub price: i16,
    pub airline: String,
    pub trip_type: i16,
    pub inserted_at: chrono::NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::trips)]
pub struct NewTrip {
    pub airport_code: String,
    pub country_code: String,
    pub city_code: String,
    pub city_name: String,
    pub depart_at: chrono::NaiveDate,
    pub arrive_at: chrono::NaiveDate, //todo: rename to return_at
    pub price: i16,
    pub airline: String,
    pub trip_type: i16,
    pub inserted_at: chrono::NaiveDateTime,
}

