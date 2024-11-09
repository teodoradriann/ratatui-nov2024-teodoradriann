use chrono::{DateTime, Local};
use futures::future::err;
use crate::api::{self, *}; // Commented out because the `api` module does not exist
use serde_json::Value;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub temp: f32,
    pub feels_like: f32,
    pub humidity: i16,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    main: Main,
    weather: Vec<WeatherInfo>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    humidity: i16,
}

#[derive(Debug, Deserialize)]
struct WeatherInfo {
    description: String,
}

pub async fn get_city_name(city: String) -> String {
    let url = format!("{}{}{}{}", CITY_FETCH_URL, city, CITY_LIMIT, API_KEY);
    let response = reqwest::get(&url).await.unwrap();
    let json = response.text().await.unwrap();

    let parsed_data: Value = serde_json::from_str(&json).expect("Bad formatted json");
    if let Some(cty) = parsed_data.get(0) {
        if let Some(city_name) = cty.get("name").and_then(|n| n.as_str()) {
            city_name.to_string()
        } else {
            "Unknown city".to_string()
        }
    } else {
        "Unknown city".to_string()
    }
}

pub async fn get_weather_forecast(city: String) -> Result<Weather, Box<dyn Error>> {
    let url = format!("{}{}{}{}", WEATHER_FETCH_URL, city, METRIC, API_KEY);
    let response = reqwest::get(&url).await?;

    let api_response : ApiResponse = serde_json::from_str(&response.text().await?).expect("Bad formatted json");

    let forecast = Weather {
        temp : api_response.main.temp,
        feels_like : api_response.main.feels_like,
        humidity : api_response.main.humidity,
        description : api_response.weather.get(0).map_or("".to_string(), |w| w.description.clone()),
    };
    Ok(forecast)
}