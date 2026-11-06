use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Weather {
    pub city: String,
    pub country: Option<String>,
    pub timezone: String,
    pub current: CurrentConditions,
    pub forecast: Vec<ForecastDay>,
}

#[derive(Debug, Clone)]
pub struct CurrentConditions {
    pub time: String,
    pub temperature_2m: f64,
    pub relative_humidity_2m: u8,
    pub apparent_temperature: f64,
    pub is_day: u8,
    pub precipitation: f64,
    pub weather_code: u16,
    pub wind_speed_10m: f64,
}

#[derive(Debug, Clone)]
pub struct ForecastDay {
    pub date: String,
    pub weather_code: u16,
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
    pub precipitation_probability_max: Option<u8>,
}

#[derive(Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize)]
struct GeocodingResult {
    name: String,
    country: Option<String>,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct ForecastResponse {
    timezone: String,
    current: CurrentResponse,
    daily: DailyResponse,
}

#[derive(Deserialize)]
struct CurrentResponse {
    time: String,
    temperature_2m: f64,
    relative_humidity_2m: u8,
    apparent_temperature: f64,
    is_day: u8,
    precipitation: f64,
    weather_code: u16,
    wind_speed_10m: f64,
}

#[derive(Deserialize)]
struct DailyResponse {
    time: Vec<String>,
    weather_code: Vec<u16>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    precipitation_probability_max: Vec<Option<u8>>,
}

pub fn fetch_weather(city: &str) -> Result<Weather> {
    let client = Client::builder()
        .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("could not create HTTP client")?;

    let location: GeocodingResult = client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[("name", city), ("count", "1"), ("language", "en"), ("format", "json")])
        .send()
        .context("could not contact the Open-Meteo geocoding service")?
        .error_for_status()
        .context("Open-Meteo geocoding request failed")?
        .json::<GeocodingResponse>()
        .context("could not parse geocoding response")?
        .results
        .and_then(|mut results| if results.is_empty() { None } else { Some(results.remove(0)) })
        .ok_or_else(|| anyhow!("no matching city found for '{city}'"))?;

    let latitude = location.latitude.to_string();
    let longitude = location.longitude.to_string();
    let response: ForecastResponse = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", latitude.as_str()),
            ("longitude", longitude.as_str()),
            (
                "current",
                "temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,weather_code,wind_speed_10m",
            ),
            (
                "daily",
                "weather_code,temperature_2m_max,temperature_2m_min,precipitation_probability_max",
            ),
            ("timezone", "auto"),
            ("forecast_days", "5"),
        ])
        .send()
        .context("could not contact the Open-Meteo forecast service")?
        .error_for_status()
        .context("Open-Meteo forecast request failed")?
        .json()
        .context("could not parse forecast response")?;

    let daily = response.daily;
    let forecast = daily
        .time
        .into_iter()
        .zip(daily.weather_code)
        .zip(daily.temperature_2m_min)
        .zip(daily.temperature_2m_max)
        .zip(daily.precipitation_probability_max)
        .map(|((((date, weather_code), temperature_2m_min), temperature_2m_max), precipitation_probability_max)| {
            ForecastDay {
                date,
                weather_code,
                temperature_2m_min,
                temperature_2m_max,
                precipitation_probability_max,
            }
        })
        .collect();

    Ok(Weather {
        city: location.name,
        country: location.country,
        timezone: response.timezone,
        current: CurrentConditions {
            time: response.current.time,
            temperature_2m: response.current.temperature_2m,
            relative_humidity_2m: response.current.relative_humidity_2m,
            apparent_temperature: response.current.apparent_temperature,
            is_day: response.current.is_day,
            precipitation: response.current.precipitation,
            weather_code: response.current.weather_code,
            wind_speed_10m: response.current.wind_speed_10m,
        },
        forecast,
    })
}

pub fn weather_description(code: u16) -> &'static str {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 | 48 => "Fog",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing rain",
        71 | 73 | 75 | 77 => "Snow",
        с80 | 81 | 82 => "Rain showers",
        85 | 86 => "Snow showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => "Unknown conditions",
    }
}

#[cfg(test)]
mod tests {
    use super::weather_description;

    #[test]
    fn describes_common_weather_codes() {
        assert_eq!(weather_description(0), "Clear sky");
        assert_eq!(weather_description(61), "Rain");
        assert_eq!(weather_description(95), "Thunderstorm");
        assert_eq!(weather_description(999), "Unknown conditions");
    }
}
