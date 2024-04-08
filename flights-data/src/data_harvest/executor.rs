use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;

/// Harvests data from third party APIs and creates `.sql` files for it to be
/// stored in the database.
pub async fn run() {
    let harvests = HashMap::from([("002_add_airports.sql", || {
        add_airports("./migrations/002_add_airports.sql")
    })]);

    let existing_sql_files: Vec<String> = fs::read_dir("./migrations/")
        .unwrap()
        .into_iter()
        .filter_map(|read_dir| read_dir.ok())
        .filter_map(|dir_entry| get_name_if_sql(dir_entry))
        .collect();

    let unexecuted = harvests
        .iter()
        .filter(|(sql_file, _)| !existing_sql_files.contains(&sql_file.to_string()))
        .collect::<HashMap<_, _>>();

    for (sql_file, function) in unexecuted {
        println!("Creating {} ...", sql_file);
        function().await;
    }
}

async fn add_airports(sql_file_name: &str) {
    let mut insert_query = String::from("INSERT INTO airports VALUES ");
    let mut search_after = None;
    loop {
        let response = super::locations_api_client::fetch_airports(search_after).await.unwrap();

        let insert_response_query = response
            .locations
            .into_iter()
            .map(|airport| {
                let escaped_name = airport.name.replace("'", "''");
                format!("('{}', '{}')", airport.code, escaped_name)
            })
            .collect::<Vec<_>>()
            .join(", ");
        insert_query.push_str(&insert_response_query);

        if response.search_after.is_none() {
            break;
        }
        search_after = response.search_after;
    }
    insert_query.push_str(";COMMIT;");

    File::create(sql_file_name)
        .unwrap()
        .write_all(insert_query.as_bytes())
        .unwrap();
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
