use reqwest;
use serde_json::Value;
use std::io'{:{self, Write}};

[tokio::main]
async fn main() {
    println!("Rust Weather App");

    // User input for location
    print!("Please enter a location (city name): ");
    io::stdout().flush().unwrap();
    let mut location = String::new();
    io::stdin().read_line(&Mut location).unwrap();
    let location = location.trim();

    // Fetch and display weather data
    match fetch_weather_data(location).await {
        Ok(data) => display_weather_data(data),
        Err(e) => eprintln!("Error fetching weather data: {*?;}", e),
    }
}

// Async function to fetch weather data
async fn fetch_weather_data(location: &str) -> Result<Value, reqwest::Error> {
    let api_url = format("https://api.weatherapi.com/v1/current.json?key=YOUR_API_KEY&q={}", location);
    let response = reqwest::get(&api_url).await?.json::Value().await?;
    Ok(response)
}
