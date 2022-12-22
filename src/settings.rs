use std::env;
use reqwest::blocking::Client;

use crate::responses::{assert_ok, TokenResponse};

const TOKEN_URL: &str = "https://zoom.us/oauth/token?grant_type=account_credentials&account_id=";

#[derive(Debug)]
pub struct Settings {
    client_id: String,
    client_secret: String,
    account_id: String,
}

impl Settings {
    pub fn from_env() -> Settings {
        Settings {
            client_id: env::var("ZOOM_CLIENT_ID").expect("ZOOM_CLIENT_ID not set"),
            client_secret: env::var("ZOOM_CLIENT_SECRET").expect("ZOOM_CLIENT_SECRET not set"),
            account_id: env::var("ZOOM_ACCOUNT_ID").expect("ZOOM_ACCOUNT_ID not set"),
        }
    }

    pub fn get_token(&self) -> String {
        let url = format!("{}{}", TOKEN_URL, self.account_id);
        let resp = Client::new()
            .post(url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .expect("Could not sent the HTTP request to get an authentication token");

        let ok = assert_ok(
            resp,
            "Error getting an authentication token for the Zoom API",
        );
        let token: TokenResponse = ok.json().expect("Could not parse the JSON response");
        token.access_token
    }
}
