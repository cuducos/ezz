use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::io::{Error, ErrorKind};

const FORMAT: &str = "%Y-%m-%d";
pub const HUMAN_FORMAT: &str = "YYYY-MM-DD";
pub const VALID_DATE_ALIASES: [&str; 9] = [
    "today",
    "tomorrow",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

fn alias_to_date(alias: &str) -> Option<NaiveDate> {
    let today = Local::now().naive_local().date();
    match alias {
        "today" => Some(today),
        "tomorrow" => Some(today + Duration::days(1)),
        _ => None,
    }
}

fn weekday_to_date(name: &str) -> Option<NaiveDate> {
    let target = match name {
        "monday" => Mon,
        "tuesday" => Tue,
        "wednesday" => Wed,
        "thursday" => Thu,
        "friday" => Fri,
        "saturday" => Sat,
        "sunday" => Sun,
        _ => return None,
    };

    let mut date = Local::now().naive_local().date();
    while date.weekday() != target {
        date += Duration::days(1);
    }
    Some(date)
}

fn date_from_str(value: &str) -> Result<NaiveDate, std::io::Error> {
    let val = value.to_lowercase();
    if let Ok(date) = NaiveDate::parse_from_str(&val, FORMAT) {
        return Ok(date);
    }
    if let Some(date) = alias_to_date(&val) {
        println!("{:?}", date); // TODO: remove
        return Ok(date);
    }
    if let Some(date) = weekday_to_date(&val) {
        return Ok(date);
    }

    Err(Error::new(
        ErrorKind::Other,
        format!(
            "Invalid date: {}. Available values are dates in the {} format or one of {}",
            value,
            HUMAN_FORMAT,
            VALID_DATE_ALIASES.join(", ")
        ),
    ))
}

pub fn parse_date(value: &str) -> Result<String, std::io::Error> {
    date_from_str(value).map(|date| date.format(FORMAT).to_string())
}
