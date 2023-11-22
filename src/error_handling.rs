pub fn handle_error(e: reqwest::Error) {
    eplintln!("API error: {}", e);
}

pub fn handle_input_error() {
    eprintln("Invalid input. Please try again.");
}
