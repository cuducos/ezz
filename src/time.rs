use chrono::NaiveTime;
use regex::Regex;
use std::io::{Error, ErrorKind};

pub const HUMAN_FORMAT: &str = "HH:MM";
const FORMAT: &str = r"(?P<hour>\d{1,2})[:hH](?P<minute>\d{2})?";

fn error(value: &str) -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        format!("Invalid time format: {}. Use {}", value, HUMAN_FORMAT,),
    )
}

fn is_valid(value: &str) -> bool {
    let with_seconds = [value, ":00"].join("");
    NaiveTime::parse_from_str(&with_seconds, "%H:%M:%S").is_ok()
}

pub fn parse_time(value: &str) -> Result<String, std::io::Error> {
    if value.ends_with(':') {
        return Err(error(value));
    }

    let re = Regex::new(FORMAT).unwrap();
    let caps = match re.captures(value) {
        Some(caps) => caps,
        None => return Err(error(value)),
    };
    if caps.name("hour").is_none() {
        return Err(error(value));
    }
    let hour = caps.name("hour").unwrap().as_str();
    let minute = match caps.name("minute") {
        Some(minute) => minute.as_str(),
        None => "00",
    };
    let formatted = format!("{:0>2}:{:0>2}", hour, minute);
    if !is_valid(&formatted) {
        return Err(error(value));
    }
    Ok(formatted)
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time("08:00").unwrap(), "08:00");
    assert_eq!(parse_time("8:05").unwrap(), "08:05");
    assert_eq!(parse_time("16:10").unwrap(), "16:10");
    assert_eq!(parse_time("08h15").unwrap(), "08:15");
    assert_eq!(parse_time("8h20").unwrap(), "08:20");
    assert_eq!(parse_time("16h25").unwrap(), "16:25");
    assert_eq!(parse_time("08h").unwrap(), "08:00");
    assert_eq!(parse_time("8h").unwrap(), "08:00");
    assert_eq!(parse_time("16h").unwrap(), "16:00");
    assert_eq!(parse_time("08H30").unwrap(), "08:30");
    assert_eq!(parse_time("8H35").unwrap(), "08:35");
    assert_eq!(parse_time("16H40").unwrap(), "16:40");
    assert_eq!(parse_time("08H").unwrap(), "08:00");
    assert_eq!(parse_time("8H").unwrap(), "08:00");
    assert_eq!(parse_time("16H").unwrap(), "16:00");
    assert!(parse_time("24h00").is_err());
    assert!(parse_time("0:60").is_err());
    assert!(parse_time("ezz").is_err());
}
