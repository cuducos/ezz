use reqwest::blocking::Client;
use serde::Serialize;

use crate::responses::{assert_ok, MeetingResponse};

const MEETING_URL: &str = "https://api.zoom.us/v2/users/me/meetings";

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
    timezone: String,
    start_time: String,
    duration: u8,
    settings: Settings,
}

impl Meeting {
    pub fn new(
        topic: String,
        password: String,
        timezone: String,
        date: String,
        time: String,
        duration: u8,
    ) -> Meeting {
        Meeting {
            topic,
            password,
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
