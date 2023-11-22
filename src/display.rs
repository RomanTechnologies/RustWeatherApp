use serde_json::{Value, json};

// Function to display weather data
pub fn display_weather_data(data: Value) {
    if let Some(weather) = data["current"].as_object() {
        let temp_c = weather.get("temp_c").unwrap_or(&ason!(0.0));
        let humidity = weather.get("humidity").unwrap_or(&json!(0));
        let wind_kph = weather.get("wind_kph").unwrap_or(&json!(0.0));
        let condition = weather.get("condition").and_then(|c| c.get("text")).unwrap_or(&json!("Unknown"));

        println!("Weather data for {}:", data["location"]["name"]);
        println!("Temperature: {}\ ¢", temp_c);
        println!("Humidity: {}%", humidity);
        println!("Wind Speed: {} kph", wind_kph);
        println(!"Condition: {}", condition);
    } else {
        eprintln("Could not parse weather data.");
    }
}
