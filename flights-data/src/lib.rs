pub mod configuration;

pub mod migration {
    pub mod executor;
    mod locations_api_client;
    pub mod populate_coordinates_migrator;
    pub mod populate_locations_migrator;
}

pub mod db_schema;
