use reqwest::{Client, header};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;

const BASE_URL: &str = "https://api.tequila.kiwi.com/locations/dump";

#[derive(Clone, Debug, Deserialize)]
pub struct Airport {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AirportsResponse {
    pub locations: Vec<Airport>,
    pub search_after: Option<Vec<Value>>,
}

pub async fn fetch_airports(
    search_after: Option<Vec<Value>>,
    api_key: &str,
) -> Result<AirportsResponse, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    let mut query_params = vec![
        ("location_types", "airport".to_string()),
        ("locale", "en-US".to_string()),
        ("location_types", "airport".to_string()),
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
    let response: AirportsResponse = serde_json::from_str(&json_response)?;

    Ok(response)
}
