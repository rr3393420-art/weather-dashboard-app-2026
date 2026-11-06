# Weather Dashboard

A small Rust command-line weather dashboard powered by the free Open-Meteo geocoding and forecast APIs. It needs no API key and shows current conditions plus a five-day forecast for a city.

## Run

```sh
cargo run -- London
```

You can also run `cargo run` and enter a city when prompted. For multi-word city names, pass the words as normal arguments, for example `cargo run -- New York`.

Temperatures are shown in °C and wind speed in km/h. The dashboard displays the location's local time zone, humidity, apparent temperature, precipitation, and forecast precipitation chance where available.

## Test

```sh
cargo test
```

Weather data is provided by [Open-Meteo](https://open-meteo.com/).
