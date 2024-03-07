use chrono::{DateTime, Utc};
use reqwest::{Client, header};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize};

const SEARCH_BASE_URL: &str = "https://api.tequila.kiwi.com/v2/search";

#[derive(Debug)]
pub struct FlightsQuery {
    pub fly_from: String,
    pub fly_to: String,
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub budget: usize,
    pub nights_in_dst_from: usize,
    pub nights_in_dst_to: usize,
    pub max_stopovers: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FlightsResponse {
    pub data: Vec<Flight>,
    #[serde(rename = "_results")]
    pub results: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flight {
    #[serde(rename = "flyFrom")]
    pub fly_from: String,
    #[serde(rename = "flyTo")]
    pub fly_to: String,
    #[serde(rename = "cityFrom")]
    pub city_from: String,
    #[serde(rename = "cityTo")]
    pub city_to: String,
    pub price: f64,
    #[serde(deserialize_with = "deserialize_date")]
    pub utc_arrival: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_date")]
    pub utc_departure: DateTime<Utc>,
    pub deep_link: String,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

pub async fn search_flights(query: FlightsQuery)
                            -> Result<FlightsResponse, Box<dyn std::error::Error>> {
    let json_response = Client::builder()
        .build()?
        .get(SEARCH_BASE_URL)
        .query(&[
            ("date_from", query.date_from.format("%d/%m/%Y").to_string()),
            ("date_to", query.date_to.format("%d/%m/%Y").to_string()),
            ("fly_from", query.fly_from),
            ("fly_to", query.fly_to),
            ("max_stopovers", query.max_stopovers.to_string()),
            ("price_to", query.budget.to_string()),
            ("nights_in_dst_from", query.nights_in_dst_from.to_string()),
            ("nights_in_dst_to", query.nights_in_dst_to.to_string()),
        ])
        .headers(get_mandatory_headers())
        .send().await?
        .text().await?;

    let response: FlightsResponse = serde_json::from_str(&json_response)?;

    return Ok(response);
}

fn get_mandatory_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("apikey", header::HeaderValue::from_static("todo: add here"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

    return headers;
}

