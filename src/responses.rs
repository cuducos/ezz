use reqwest::blocking::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct MeetingResponse {
    pub join_url: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    pub message: String,
}

pub fn assert_ok(resp: Response, action: &str) -> Response {
    if resp.status().is_success() {
        return resp;
    }
    let error: ErrorResponse = resp
        .json()
        .expect("Could not parse the JSON from an error response");
    println!("Error {} via the Zoom API: {}", action, error.message);
    std::process::exit(1);
}
