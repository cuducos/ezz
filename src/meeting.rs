use rand::Rng;
use reqwest::blocking::Client;
use serde::Serialize;

use crate::responses::{assert_ok, MeetingResponse};

const MEETING_URL: &str = "https://api.zoom.us/v2/users/me/meetings";

fn random_password() -> String {
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| rng.gen_range(0..=9))
        .map(|n| format!("{}", n))
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_random_password() {
    let password = random_password();
    assert_eq!(password.len(), 6);
    assert!(password.chars().all(|c| c.is_numeric()));
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
