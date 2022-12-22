use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
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

#[cfg(not(test))]
mod today {
    use chrono::{Local, NaiveDate};

    pub fn today() -> NaiveDate {
        Local::now().naive_local().date()
    }
}

#[cfg(not(test))]
use today::today;

#[cfg(test)]
mod mock_today {
    use chrono::NaiveDate;

    pub fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2022, 12, 28).unwrap()
    }
}

#[cfg(test)]
use mock_today::today;

fn alias_to_date(alias: &str) -> Option<NaiveDate> {
    match alias {
        "today" => Some(today()),
        "tomorrow" => Some(today() + Duration::days(1)),
        _ => None,
    }
}

#[test]
fn test_alias_to_date() {
    assert_eq!(
        alias_to_date("today"),
        NaiveDate::from_ymd_opt(2022, 12, 28),
    );
    assert_eq!(
        alias_to_date("tomorrow"),
        NaiveDate::from_ymd_opt(2022, 12, 29),
    );
    assert!(alias_to_date("ezz").is_none());
}

fn weekday_to_date(weekday: Weekday) -> Option<NaiveDate> {
    let mut date = today();
    while date.weekday() != weekday {
        date += Duration::days(1);
    }
    Some(date)
}

#[test]
fn test_weekday_to_date() {
    assert_eq!(weekday_to_date(Mon), NaiveDate::from_ymd_opt(2023, 1, 2));
    assert_eq!(weekday_to_date(Tue), NaiveDate::from_ymd_opt(2023, 1, 3));
    assert_eq!(weekday_to_date(Wed), NaiveDate::from_ymd_opt(2022, 12, 28));
    assert_eq!(weekday_to_date(Thu), NaiveDate::from_ymd_opt(2022, 12, 29));
    assert_eq!(weekday_to_date(Fri), NaiveDate::from_ymd_opt(2022, 12, 30));
    assert_eq!(weekday_to_date(Sat), NaiveDate::from_ymd_opt(2022, 12, 31));
    assert_eq!(weekday_to_date(Sun), NaiveDate::from_ymd_opt(2023, 1, 1));
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

#[test]
fn test_date_from_str() {
    assert_eq!(
        date_from_str("2023-02-08").ok(),
        NaiveDate::from_ymd_opt(2023, 2, 8),
    );
    assert_eq!(
        date_from_str("today").unwrap(),
        NaiveDate::from_ymd_opt(2022, 12, 28).unwrap()
    );
    assert_eq!(
        date_from_str("tomorrow").unwrap(),
        NaiveDate::from_ymd_opt(2022, 12, 29).unwrap()
    );
    assert_eq!(
        date_from_str("monday").ok(),
        NaiveDate::from_ymd_opt(2023, 1, 2)
    );
    assert_eq!(
        date_from_str("tuesday").ok(),
        NaiveDate::from_ymd_opt(2023, 1, 3)
    );
    assert_eq!(
        date_from_str("Wednesday").ok(),
        NaiveDate::from_ymd_opt(2022, 12, 28)
    );
    assert_eq!(
        date_from_str("thursday").ok(),
        NaiveDate::from_ymd_opt(2022, 12, 29)
    );
    assert_eq!(
        date_from_str("friday").ok(),
        NaiveDate::from_ymd_opt(2022, 12, 30)
    );
    assert_eq!(
        date_from_str("saturday").ok(),
        NaiveDate::from_ymd_opt(2022, 12, 31)
    );
    assert!(date_from_str("ezz").is_err());
    assert_eq!(
        date_from_str("sunday").ok(),
        NaiveDate::from_ymd_opt(2023, 1, 1)
    );
}

pub fn parse_date(value: &str) -> Result<String, std::io::Error> {
    date_from_str(value).map(|date| date.format(FORMAT).to_string())
}

#[test]
fn test_parse_date() {
    assert_eq!(parse_date("2023-02-08").unwrap(), "2023-02-08");
    assert_eq!(parse_date("today").unwrap(), "2022-12-28");
    assert_eq!(parse_date("tomorrow").unwrap(), "2022-12-29");
    assert_eq!(parse_date("monday").unwrap(), "2023-01-02");
    assert_eq!(parse_date("tuesday").unwrap(), "2023-01-03");
    assert_eq!(parse_date("Wednesday").unwrap(), "2022-12-28");
    assert_eq!(parse_date("thursday").unwrap(), "2022-12-29");
    assert_eq!(parse_date("friday").unwrap(), "2022-12-30");
    assert_eq!(parse_date("saturday").unwrap(), "2022-12-31");
    assert_eq!(parse_date("sunday").unwrap(), "2023-01-01");
    assert!(parse_date("ezz").is_err());
}
