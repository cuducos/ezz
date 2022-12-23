use super::Meeting;
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub enum ClientError {
    MissingEnvVar(String),
    RequestError(reqwest::Error),
    ZoomError(String),
    SerializerError(serde_json::Error),
    DeserializerError(reqwest::Error),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::MissingEnvVar(key) => write!(f, "Environment variable {} not found.", key),
            ClientError::RequestError(e) => write!(f, "HTTP Request error: {}", e),
            ClientError::ZoomError(msg) => write!(f, "Zoom API returned an error: {}", msg),
            ClientError::SerializerError(e) => {
                write!(f, "Error preparing data to send to Zoom API: {}", e)
            }
            ClientError::DeserializerError(e) => {
                write!(f, "Error parsing response from Zoom API: {}", e)
            }
        }
    }
}

impl Authentication {
    fn from_env() -> Result<Authentication, ClientError> {
        let names: [&str; 3] = ["ZOOM_ACCOUNT_ID", "ZOOM_CLIENT_ID", "ZOOM_CLIENT_SECRET"];
        names
            .iter()
            .map(|k| std::env::var(k).map_err(|_| ClientError::MissingEnvVar(k.to_string())))
            .collect::<Result<Vec<String>, ClientError>>()
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
    pub fn new() -> Result<Self, ClientError> {
        Authentication::from_env()
            .map(|auth| Self::from(auth, TOKEN_URL.to_string(), MEETING_URL.to_string()))
    }

    pub fn from(auth: Authentication, token_url: String, meeting_url: String) -> Self {
        Self {
            client: Client::new(),
            auth,
            token_url,
            meeting_url,
        }
    }

    fn send(&self, request: reqwest::blocking::RequestBuilder) -> Result<Response, ClientError> {
        let resp = request.send().map_err(ClientError::RequestError)?;
        if !resp.status().is_success() {
            let error: ErrorResponse = resp.json().map_err(ClientError::DeserializerError)?;
            return Err(ClientError::ZoomError(error.message));
        }
        Ok(resp)
    }

    fn token(&self) -> Result<String, ClientError> {
        let req = self
            .client
            .post(format!("{}{}", self.token_url, self.auth.account_id).as_str())
            .basic_auth(&self.auth.client_id, Some(&self.auth.client_secret));
        let resp = self.send(req)?;
        let token: TokenResponse = resp.json().map_err(ClientError::DeserializerError)?;
        Ok(token.access_token)
    }

    pub fn save(&self, meeting: &Meeting) -> Result<String, ClientError> {
        let body = serde_json::to_string(meeting).map_err(ClientError::SerializerError)?;
        let token = self.token()?;
        let req = self
            .client
            .post(self.meeting_url.as_str())
            .header("authorization", format!("Bearer {}", token))
            .header("content-type", "application/json")
            .body(body);
        let resp = self.send(req)?;
        let meeting: MeetingResponse = resp.json().map_err(ClientError::DeserializerError)?;
        Ok(meeting.join_url)
    }
}
