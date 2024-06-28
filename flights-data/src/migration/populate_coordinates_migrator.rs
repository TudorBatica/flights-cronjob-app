use std::fs::OpenOptions;
use std::io::prelude::*;

use async_trait::async_trait;
use sea_query::{Expr, PostgresQueryBuilder, Query};

use crate::configuration::Settings;
use crate::db_schema::Locations;
use crate::migration::executor::{Migration, MigrationConstructor};
use crate::migration::locations_api_client;
use crate::migration::locations_api_client::{Airport, City, LocationType};

inventory::submit! {
    MigrationConstructor(|| {
        Box::new(PopulateLocationsMigration {})
    })
}

pub struct PopulateLocationsMigration {}

#[async_trait]
impl Migration for PopulateLocationsMigration {
    fn output_file_name(&self) -> &str {
        "010_populate_coordinates_columns.sql"
    }

    async fn run(&self, settings: &Settings) {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(format!("./migrations/{}", self.output_file_name()))
            .unwrap();

        let cities: Vec<City> =
            locations_api_client::get_locations(LocationType::City, &settings.kiwi_api_key).await;
        for city in cities {
            let coordinates = city.location.unwrap();
            write!(
                &mut file,
                "{};\n",
                update_coordinates_statement(city.id, coordinates.lat, coordinates.lon)
            )
            .unwrap();
        }

        let airports: Vec<Airport> =
            locations_api_client::get_locations(LocationType::Airport, &settings.kiwi_api_key)
                .await;
        for airport in airports {
            write!(
                &mut file,
                "{};\n",
                update_coordinates_statement(
                    airport.id,
                    airport.location.lat,
                    airport.location.lon
                )
            )
            .unwrap();
        }
    }
}

fn update_coordinates_statement(location_id: String, lat: f64, lon: f64) -> String {
    Query::update()
        .table(Locations::Table)
        .values([
            (Locations::Latitude, lat.into()),
            (Locations::Longitude, lon.into()),
        ])
        .and_where(Expr::col(Locations::Id).eq(location_id))
        .to_string(PostgresQueryBuilder)
}
