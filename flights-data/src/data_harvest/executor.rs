use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;

use crate::configuration::Settings;
use crate::data_harvest::locations_api_client::{fetch_locations, LocationType};

/// Harvests data from third party APIs and generates `.sql` files for it to be
/// stored in the database.
pub async fn run(settings: &Settings) {
    let files_to_generate = vec![
        "002_add_airports.sql",
        "003_add_countries.sql",
    ];

    let existing_sql_files: Vec<String> = fs::read_dir("./migrations/")
        .unwrap()
        .into_iter()
        .filter_map(|read_dir| read_dir.ok())
        .filter_map(|dir_entry| get_name_if_sql(dir_entry))
        .collect();

    let unexecuted = files_to_generate.iter()
        .filter(|file| !existing_sql_files.contains(&file.to_string()))
        .collect::<Vec<&&str>>();

    for new_harvest in unexecuted {
        match new_harvest {
            &"002_add_airports.sql" => add_airports("./migrations/002_add_airports.sql", settings).await,
            &"003_add_countries.sql" => add_countries("./migrations/003_add_countries.sql", settings).await,
            _ => { panic!("No handler for {}", new_harvest) }
        }
    }
}

async fn add_locations(location_type: LocationType, sql_file_name: &str, settings: &Settings) {
    let table = match location_type {
        LocationType::Airport => { "airports" }
        LocationType::Country => { "countries" }
    };

    let mut insert_query = format!("INSERT INTO {} VALUES ", table);
    let mut search_after = None;
    loop {
        let response = fetch_locations(&location_type, search_after, &settings.kiwi_api_key)
            .await
            .unwrap();

        let insert_response_query = response
            .locations
            .into_iter()
            .map(|location| {
                let escaped_name = location.name.replace("'", "''");
                format!("('{}', '{}')", location.code, escaped_name)
            })
            .collect::<Vec<_>>()
            .join(", ");
        insert_query.push_str(&insert_response_query);

        if response.search_after.is_none() {
            break;
        }
        search_after = response.search_after;
    }
    insert_query.push_str(";");

    File::create(sql_file_name)
        .unwrap()
        .write_all(insert_query.as_bytes())
        .unwrap();
}

async fn add_airports(sql_file_name: &str, settings: &Settings) {
    add_locations(LocationType::Airport, sql_file_name, settings).await;
}

async fn add_countries(sql_file_name: &str, settings: &Settings) {
    add_locations(LocationType::Country, sql_file_name, settings).await;
}

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
