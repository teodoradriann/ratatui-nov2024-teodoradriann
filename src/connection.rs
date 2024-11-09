use chrono::{DateTime, Local};

struct CityInfo {
    // TODO: define elements in the structure
}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
pub fn get_data(city: String) {
    match reqwest::blocking::get("") {
        Ok(response) => {
            // Check status code
            // Parse response
        },
        Err(error) => {
            // Handle error
        }
    }
}