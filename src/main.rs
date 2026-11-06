mod cache;
mod weather;

use anyhow::{bail, Context, Result};
use cache::Cache;
use std::io::{self, Write};
use weather::{fetch_weather, Weather};

fn main() -> Result<()> {
    let mut city = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if city.trim().is_empty() {
        print!("Enter a city: ");
        io::stdout().flush().context("could not display prompt")?;
        io::stdin()
            .read_line(&mut city)
            .context("could not read city name")?;
        city = city.trim().to_owned();
    }

    if city.is_empty() {
        bail!("Please provide a city name, for example: cargo run -- London");
    }

    let cache = Cache::new();
    let cache_key = city.to_lowercase();
    let weather = match cache.get(&cache_key) {
        Some(weather) => weather,
        None => {
            let weather = fetch_weather(&city)
                .with_context(|| format!("could not load weather for {city}"))?;
            cache.set(cache_key, weather.clone());
            weather
        }
    };

    render_dashboard(&weather);
    Ok(())
}

fn render_dashboard(weather: &Weather) {
    let country = weather
        .country
        .as_deref()
        .map(|country| format!(", {country}"))
        .unwrap_or_default();

    println!("\nWeather dashboard — {}{}", weather.city, country);
    println!("{} | Local time: {}", weather.timezone, weather.current.time);
    println!("{}", "─".repeat(58));
    println!(
        "Now: {}  {:.1}°C (feels like {:.1}°C)",
        weather_description(weather.current.weather_code),
        weather.current.temperature_2m,
        weather.current.apparent_temperature
    );
    println!(
        "Humidity: {}%   Wind: {:.1} km/h   Precipitation: {:.1} mm   {}",
        weather.current.relative_humidity_2m,
        weather.current.wind_speed_10m,
        weather.current.precipitation,
        if weather.current.is_day == 1 { "Day" } else { "Night" }
    );
    println!("\n5-day forecast");
    println!("{:<13} {:<23} {:>8} {:>8} {:>10}", "Date", "Conditions", "Low", "High", "Rain %");

    for day in &weather.forecast {
        let precipitation = day
            .precipitation_probability_max
            .map(|chance| format!("{chance}%"))
            .unwrap_or_else(|| "—".to_owned());
        println!(
            "{:<13} {:<23} {:>7.1}° {:>7.1}° {:>10}",
            day.date,
            weather_description(day.weather_code),
            day.temperature_2m_min,
            day.temperature_2m_max,
            precipitation
        );
    }
}
