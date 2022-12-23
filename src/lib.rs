use rand::Rng;
use serde::Serialize;

pub mod client;
pub mod date;
pub mod time;

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
struct MeetingSettings {
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
    settings: MeetingSettings,
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
            settings: MeetingSettings {
                host_video: true,
                participant_video: false,
                waiting_room: true,
            },
        }
    }
}
