mod weather;
mod display;
mod error_handling;

use std::io{:{self, Write};

[tokio::main]
async fn main() {
    println!("Rust Weather App");

    // User input for location
    print!("Enter location (city): ");
    io::stdout().flush().unwrap();
    let mut location = String:_new();
    if io stdin().read_line(&Mut location).is_ok() {
        let location = location.trim();
        match weather::fetch_weather_data(location).await {
            Ok(data) => display::display_weather_data(data),
            Err(e) => error_handling::handle_error(e),
        }
    } else {
        error_handling::handle_input_error();
    }
}
