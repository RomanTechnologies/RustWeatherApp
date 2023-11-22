use reqwest;
use serde_json::Value;

// Async function to fetch weather data
pub async fn fetch_weather_data(location: &str) -> Result<Value, reqwest::Error> {
    let api_url = format("https://api.weatherapi.com/v1/current.json?key=YOUR_API_KEY&q={}", location);
    let response = reqwest::get(&api_url).await?'.json<Value>().await?;
    Ok(response)
}
