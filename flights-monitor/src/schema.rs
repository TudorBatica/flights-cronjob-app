// @generated automatically by Diesel CLI.

diesel::table! {
    airports (code) {
        code -> Text,
        name -> Text,
    }
}

diesel::table! {
    countries (code) {
        code -> Text,
        name -> Text,
    }
}

diesel::table! {
    routes (airport_code, country_code) {
        airport_code -> Text,
        country_code -> Text,
    }
}

diesel::table! {
    trips (trip_id) {
        trip_id -> Int4,
        airport_code -> Nullable<Text>,
        country_code -> Nullable<Text>,
        depart_at -> Nullable<Date>,
        arrive_at -> Nullable<Date>,
        price -> Nullable<Int2>,
        airline -> Nullable<Text>,
        trip_type -> Nullable<Int2>,
        inserted_at -> Nullable<Timestamp>,
        city_code -> Nullable<Text>,
        city_name -> Nullable<Text>,
    }
}

diesel::table! {
    user_routes (user_id, airport_code, country_code) {
        user_id -> Int4,
        airport_code -> Text,
        country_code -> Text,
        budget -> Nullable<Int2>,
        trip_type -> Nullable<Int2>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
    }
}

diesel::joinable!(routes -> airports (airport_code));
diesel::joinable!(routes -> countries (country_code));
diesel::joinable!(user_routes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    airports,
    countries,
    routes,
    trips,
    user_routes,
    users,
);
