use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use serde::Deserialize;
use serde_json::Value;

const BASE_URL: &str = "https://api.tequila.kiwi.com/locations/dump";

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LocationsResponse {
    pub locations: Vec<Location>,
    pub search_after: Option<Vec<Value>>,
}

#[derive(Debug)]
pub enum LocationType {
    Airport,
    Country,
}

pub async fn fetch_locations(
    location_type: &LocationType,
    search_after: Option<Vec<Value>>,
    api_key: &str,
) -> Result<LocationsResponse, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    let location_type = match location_type {
        LocationType::Airport => "airport".to_string(),
        LocationType::Country => "country".to_string(),
    };

    let mut query_params = vec![
        ("location_types", location_type),
        ("locale", "en-US".to_string()),
        ("limit", "15000".to_string()),
    ];

    if let Some(search_params) = search_after {
        query_params.push((
            "search_after",
            search_params[0].as_i64().unwrap().to_string(),
        ));
        query_params.push((
            "search_after",
            search_params[1].as_str().unwrap().to_string(),
        ));
    }

    let json_response = Client::builder()
        .build()?
        .get(BASE_URL)
        .query(&query_params)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let response: LocationsResponse = serde_json::from_str(&json_response)?;

    Ok(response)
}
