use super::Meeting;
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use std::io::{Error, ErrorKind};

const MEETING_URL: &str = "https://api.zoom.us/v2/users/me/meetings";
const TOKEN_URL: &str = "https://zoom.us/oauth/token?grant_type=account_credentials&account_id=";

#[derive(Deserialize)]
struct TokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
struct MeetingResponse {
    pub join_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub struct Authentication {
    account_id: String,
    client_id: String,
    client_secret: String,
}

impl Authentication {
    fn from_env() -> Result<Authentication, std::env::VarError> {
        let names: [&str; 3] = ["ZOOM_ACCOUNT_ID", "ZOOM_CLIENT_ID", "ZOOM_CLIENT_SECRET"];
        names
            .iter()
            .map(std::env::var)
            .collect::<Result<Vec<String>, std::env::VarError>>()
            .map(|args| Authentication::new(args[0].clone(), args[1].clone(), args[2].clone()))
    }

    pub fn new(account_id: String, client_id: String, client_secret: String) -> Authentication {
        Authentication {
            account_id,
            client_id,
            client_secret,
        }
    }
}

pub struct Zoom {
    auth: Authentication,
    client: Client,
    token_url: String,
    meeting_url: String,
}

impl Zoom {
    pub fn new() -> Result<Self, std::io::Error> {
        Authentication::from_env()
            .map(|auth| Self::from(auth, TOKEN_URL.to_string(), MEETING_URL.to_string()))
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }

    pub fn from(auth: Authentication, token_url: String, meeting_url: String) -> Self {
        Self {
            client: Client::new(),
            auth,
            token_url,
            meeting_url,
        }
    }

    fn send(&self, request: reqwest::blocking::RequestBuilder) -> Result<Response, ErrorResponse> {
        request.send().map_err(|e| ErrorResponse {
            message: e.to_string(),
        })
    }

    fn token(&self) -> Result<String, ErrorResponse> {
        let url = format!("{}{}", self.token_url, self.auth.account_id);
        let resp = self.send(
            self.client
                .post(url.as_str())
                .basic_auth(&self.auth.client_id, Some(&self.auth.client_secret)),
        )?;

        match resp.json() {
            Ok(data) => {
                let token: TokenResponse = data;
                Ok(token.access_token)
            }
            Err(e) => Err(ErrorResponse {
                message: e.to_string(),
            }),
        }
    }

    pub fn save(&self, meeting: &Meeting) -> Result<String, ErrorResponse> {
        let body = match serde_json::to_string(meeting) {
            Ok(body) => body,
            Err(e) => {
                return Err(ErrorResponse {
                    message: e.to_string(),
                })
            }
        };

        let token = self.token()?;
        let resp = self.send(
            self.client
                .post(self.meeting_url.as_str())
                .header("authorization", format!("Bearer {}", token))
                .header("content-type", "application/json")
                .body(body),
        )?;

        let meeting: MeetingResponse = resp.json().map_err(|e| ErrorResponse {
            message: e.to_string(),
        })?;
        Ok(meeting.join_url)
    }
}
