use std::fs::DirEntry;

use sea_query::{PostgresQueryBuilder, Query};

use crate::configuration::Settings;
use crate::data_harvest::locations_api_client;
use crate::data_harvest::locations_api_client::{
    Airport, AutonomousTerritory, City, Continent, Country, LocationType, Region, Subdivision,
};
use crate::db_schema::{LocationTypeEnum, Locations};

/// Harvests data from third party APIs and generates `.sql` files for it to be
/// stored in the database.
pub async fn run(settings: &Settings) {
    // println!("{}", build_insert_continents_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_regions_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_countries_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_autonomous_territories_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_subdivisions_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_cities_query(&settings.kiwi_api_key).await);
    // println!("{}", build_insert_airports_query(&settings.kiwi_api_key).await);
}

async fn build_insert_continents_query(kiwi_api_key: &str) -> String {
    let continents: Vec<Continent> =
        locations_api_client::get_locations(LocationType::Continent, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::LocationType,
    ]);
    continents.into_iter().for_each(|continent| {
        query.values_panic(vec![
            continent.id.into(),
            continent.name.into(),
            LocationTypeEnum::Continent.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_regions_query(kiwi_api_key: &str) -> String {
    let regions: Vec<Region> =
        locations_api_client::get_locations(LocationType::Region, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::LocationType,
    ]);
    regions.into_iter().for_each(|region| {
        query.values_panic(vec![
            region.id.into(),
            region.name.into(),
            region.continent.map(|cont| cont.id).into(),
            LocationTypeEnum::Region.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_countries_query(kiwi_api_key: &str) -> String {
    let countries: Vec<Country> =
        locations_api_client::get_locations(LocationType::Country, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::RegionId,
        Locations::LocationType,
    ]);
    countries.into_iter().for_each(|country| {
        query.values_panic(vec![
            country.id.into(),
            country.name.into(),
            country.continent.map(|country| country.id).into(),
            country.region.map(|region| region.id).into(),
            LocationTypeEnum::Country.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_autonomous_territories_query(kiwi_api_key: &str) -> String {
    let territories: Vec<AutonomousTerritory> =
        locations_api_client::get_locations(LocationType::AutonomousTerritory, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::RegionId,
        Locations::CountryId,
        Locations::LocationType,
    ]);
    territories.into_iter().for_each(|territory| {
        query.values_panic(vec![
            territory.id.into(),
            territory.name.into(),
            territory.continent.map(|cont| cont.id).into(),
            territory.region.map(|reg| reg.id).into(),
            territory.country.map(|country| country.id).into(),
            LocationTypeEnum::AutonomousTerritory.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_subdivisions_query(kiwi_api_key: &str) -> String {
    let subdivisions: Vec<Subdivision> =
        locations_api_client::get_locations(LocationType::Subdivision, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::RegionId,
        Locations::CountryId,
        Locations::LocationType,
    ]);
    subdivisions.into_iter().for_each(|subdv| {
        query.values_panic(vec![
            subdv.id.into(),
            subdv.name.into(),
            subdv.continent.map(|cont| cont.id).into(),
            subdv.region.map(|region| region.id).into(),
            subdv.country.map(|country| country.id).into(),
            LocationTypeEnum::Subdivision.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_cities_query(kiwi_api_key: &str) -> String {
    let cities: Vec<City> =
        locations_api_client::get_locations(LocationType::City, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::RegionId,
        Locations::CountryId,
        Locations::SubdivisionId,
        Locations::LocationType,
    ]);
    cities.into_iter().for_each(|city| {
        query.values_panic(vec![
            city.id.into(),
            city.name.into(),
            city.continent.map(|cont| cont.id).into(),
            city.region.map(|region| region.id).into(),
            city.country
                .map(|country| {
                    // kiwi appears to be sending ZZ country code for cities with unknown/disputed country
                    if country.id == "ZZ" {
                        None
                    } else {
                        Some(country.id)
                    }
                })
                .flatten()
                .into(),
            city.subdivision.map(|subdivision| subdivision.id).into(),
            LocationTypeEnum::City.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn build_insert_airports_query(kiwi_api_key: &str) -> String {
    let airports: Vec<Airport> =
        locations_api_client::get_locations(LocationType::Airport, kiwi_api_key).await;
    let mut query = Query::insert();
    query.into_table(Locations::Table).columns(vec![
        Locations::Id,
        Locations::Name,
        Locations::ContinentId,
        Locations::RegionId,
        Locations::CountryId,
        Locations::SubdivisionId,
        Locations::CityId,
        Locations::LocationType,
    ]);
    airports.into_iter().for_each(|airport| {
        query.values_panic(vec![
            airport.id.into(),
            airport.name.into(),
            airport
                .city
                .as_ref()
                .map(|city| city.continent.as_ref().map(|cont| cont.id.as_ref()))
                .flatten()
                .into(),
            airport
                .city
                .as_ref()
                .map(|city| city.region.as_ref().map(|reg| reg.id.as_ref()))
                .flatten()
                .into(),
            airport
                .city
                .as_ref()
                .map(|city| {
                    city.country.as_ref().map(|country| {
                        // kiwi appears to be sending ZZ country code for cities with unknown/disputed country
                        if country.id == "ZZ" {
                            None
                        } else {
                            Some(country.id.as_ref())
                        }
                    })
                })
                .flatten()
                .flatten()
                .into(),
            airport
                .city
                .as_ref()
                .map(|city| city.subdivision.as_ref().map(|country| country.id.as_ref()))
                .flatten()
                .into(),
            airport.city.as_ref().map(|city| city.id.as_ref()).into(),
            LocationTypeEnum::Airport.to_string().into(),
        ]);
    });

    return query.to_string(PostgresQueryBuilder) + ";";
}

async fn add_all_locations(sql_file_name: &str, settings: &Settings) {}

fn get_name_if_sql(dir_entry: DirEntry) -> Option<String> {
    let is_file = dir_entry
        .file_type()
        .is_ok_and(|file_type| file_type.is_file());
    let is_sql = dir_entry
        .file_name()
        .to_str()
        .map_or(false, |name| String::from(name).ends_with(".sql"));

    if is_file && is_sql {
        dir_entry.file_name().to_str().map(|str| str.to_string())
    } else {
        None
    }
}
