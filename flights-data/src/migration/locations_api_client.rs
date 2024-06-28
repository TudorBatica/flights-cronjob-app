use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::fmt;

const BASE_URL: &str = "https://api.tequila.kiwi.com/locations/dump";

#[derive(Clone, Debug, Deserialize)]
pub struct Continent {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub continent: Option<Continent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Country {
    pub id: String,
    pub name: String,
    pub region: Option<Region>,
    pub continent: Option<Continent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Subdivision {
    pub id: String,
    pub name: String,
    pub continent: Option<Continent>,
    pub country: Option<Country>,
    pub region: Option<Region>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AutonomousTerritory {
    pub id: String,
    pub name: String,
    pub continent: Option<Continent>,
    pub country: Option<Country>,
    pub region: Option<Region>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct City {
    pub id: String,
    pub name: String,
    pub autonomous_territory: Option<AutonomousTerritory>,
    pub country: Option<Country>,
    pub continent: Option<Continent>,
    pub subdivision: Option<Subdivision>,
    pub region: Option<Region>,
    pub location: Option<Location>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Airport {
    pub id: String,
    pub name: String,
    pub autonomous_territory: Option<AutonomousTerritory>,
    pub country: Option<Country>,
    pub continent: Option<Continent>,
    pub subdivision: Option<Subdivision>,
    pub region: Option<Region>,
    pub city: Option<City>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LocationResponse<T> {
    pub locations: Vec<T>,
    pub search_after: Option<Vec<Value>>,
}

#[derive(Debug)]
pub enum LocationType {
    Airport,
    AutonomousTerritory,
    City,
    Country,
    Continent,
    Subdivision,
    Region,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LocationType::Airport => "airport",
            LocationType::Country => "country",
            LocationType::AutonomousTerritory => "autonomous_territory",
            LocationType::City => "city",
            LocationType::Continent => "continent",
            LocationType::Region => "region",
            LocationType::Subdivision => "subdivision",
        };
        write!(f, "{}", s)
    }
}

pub async fn get_locations<T>(location_type: LocationType, api_key: &str) -> Vec<T>
where
    T: DeserializeOwned + Clone,
{
    let mut locations: Vec<T> = vec![];
    let mut search_after = None;
    loop {
        let response = fetch_locations_from_api(&location_type, search_after, api_key)
            .await
            .unwrap();
        let mut response: LocationResponse<T> = serde_json::from_str(&response).unwrap();
        locations.append(&mut response.locations);
        if let Some(search_pagination) = response.search_after {
            search_after = Some(search_pagination);
        } else {
            return locations;
        }
    }
}

async fn fetch_locations_from_api(
    location_type: &LocationType,
    search_after: Option<Vec<Value>>,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("apikey", api_key.parse().unwrap());
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    let mut query_params = vec![
        ("location_types", location_type.to_string()),
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

    return Ok(json_response);
}
