use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use std::io::{Error, ErrorKind};

const FORMAT: &str = "%Y-%m-%d";
pub const HUMAN_FORMAT: &str = "YYYY-MM-DD";
pub const ALIASES: [&str; 9] = [
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

fn weekday_to_date(weekday: Weekday) -> Option<NaiveDate> {
    let mut date = Local::now().naive_local().date();
    while date.weekday() != weekday {
        date += Duration::days(1);
    }
    Some(date)
}

struct Parser(Option<Weekday>);

impl Parser {
    fn for_alias() -> Parser {
        Parser(None)
    }
    fn for_weekday(weekday: Weekday) -> Parser {
        Parser(Some(weekday))
    }
    fn parse(&self, value: &str) -> Option<NaiveDate> {
        match self.0 {
            Some(weekday) => weekday_to_date(weekday),
            None => alias_to_date(value),
        }
    }
}

fn date_from_str(value: &str) -> Result<NaiveDate, std::io::Error> {
    let val = value.to_lowercase();
    if let Ok(date) = NaiveDate::parse_from_str(&val, FORMAT) {
        return Ok(date);
    }

    let parsers: [Parser; 9] = [
        Parser::for_alias(),
        Parser::for_alias(),
        Parser::for_weekday(Mon),
        Parser::for_weekday(Tue),
        Parser::for_weekday(Wed),
        Parser::for_weekday(Thu),
        Parser::for_weekday(Fri),
        Parser::for_weekday(Sat),
        Parser::for_weekday(Sun),
    ];

    ALIASES
        .iter()
        .position(|&idx| idx == val)
        .map(|idx| &parsers[idx])
        .and_then(|parser| parser.parse(&val))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Invalid date format. Use {} or one of: {}",
                    HUMAN_FORMAT,
                    ALIASES.join(", ")
                ),
            )
        })
}

pub fn parse_date(value: &str) -> Result<String, std::io::Error> {
    date_from_str(value).map(|date| date.format(FORMAT).to_string())
}
