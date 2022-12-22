use rand::Rng;
use reqwest::blocking::Client;
use serde::Serialize;

use crate::responses::{assert_ok, MeetingResponse};

const MEETING_URL: &str = "https://api.zoom.us/v2/users/me/meetings";
const PASSWORD_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789*-_@";

fn random_password() -> String {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(6..=10);
    (0..size)
        .map(|_| rng.gen_range(0..PASSWORD_CHARS.len()))
        .map(|n| PASSWORD_CHARS.chars().nth(n).unwrap())
        .collect::<String>()
}

#[test]
fn test_random_password() {
        let password = random_password();
        assert!(password.len() <= 10);
        assert!(password.len() >= 6);
        for char in password.chars() {
            assert!(PASSWORD_CHARS.contains(char));
        }
}

#[derive(Serialize)]
struct Settings {
    host_video: bool,
    participant_video: bool,
    waiting_room: bool,
}

#[derive(Serialize)]
pub struct Meeting {
    topic: String,
    password: String,
    timezone: Option<String>,
    start_time: String,
    duration: u16,
    settings: Settings,
}

impl Meeting {
    pub fn new(
        topic: String,
        password: Option<String>,
        timezone: Option<String>,
        date: String,
        time: String,
        duration: u16,
    ) -> Meeting {
        Meeting {
            topic,
            password: password.unwrap_or_else(random_password),
            timezone,
            start_time: format!("{}T{}:00", date, time),
            duration,
            settings: Settings {
                host_video: true,
                participant_video: false,
                waiting_room: true,
            },
        }
    }

    pub fn save(&self, token: String) -> String {
        let resp = Client::new()
            .post(MEETING_URL)
            .header("authorization", format!("Bearer {}", token))
            .header("content-type", "application/json")
            .body(serde_json::to_string(self).unwrap())
            .send()
            .expect("Could not sent the HTTP request to create the meeting");

        let ok = assert_ok(resp, "Error creating the meeting via the Zoom API");
        let meeting: MeetingResponse = ok
            .json()
            .expect("Could not parse the JSON from a successul response");
        meeting.join_url
    }
}
