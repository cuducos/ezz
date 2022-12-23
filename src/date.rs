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

struct Parser {
    base: NaiveDate,
    target: Option<Weekday>,
}

impl Parser {
    fn for_alias(base: NaiveDate) -> Self {
        Self { base, target: None }
    }

    fn for_weekday(base: NaiveDate, target: Weekday) -> Self {
        Self {
            base,
            target: Some(target),
        }
    }

    fn alias(&self, name: &str) -> Option<NaiveDate> {
        match name {
            "today" => Some(self.base),
            "tomorrow" => Some(self.base + Duration::days(1)),
            _ => None,
        }
    }

    fn weekday(&self) -> Option<NaiveDate> {
        let mut date = self.base;
        while date.weekday() != self.target? {
            date += Duration::days(1);
        }
        Some(date)
    }

    fn parse(&self, name: &str) -> Option<NaiveDate> {
        match self.target {
            Some(_) => self.weekday(),
            None => self.alias(name),
        }
    }
}

#[test]
fn test_parser_alias() {
    let today = NaiveDate::from_ymd_opt(2022, 12, 28).unwrap();
    let parser = Parser::for_alias(today);
    assert!(parser.alias("ezz").is_none());
    assert_eq!(parser.alias("today"), NaiveDate::from_ymd_opt(2022, 12, 28),);
    assert_eq!(
        parser.alias("tomorrow"),
        NaiveDate::from_ymd_opt(2022, 12, 29),
    );
}

#[test]
fn test_parser_weekday() {
    let today = NaiveDate::from_ymd_opt(2022, 12, 28).unwrap();
    assert_eq!(
        Parser::for_weekday(today, Mon).weekday(),
        NaiveDate::from_ymd_opt(2023, 1, 2)
    );
    assert_eq!(
        Parser::for_weekday(today, Tue).weekday(),
        NaiveDate::from_ymd_opt(2023, 1, 3)
    );
    assert_eq!(
        Parser::for_weekday(today, Wed).weekday(),
        NaiveDate::from_ymd_opt(2022, 12, 28)
    );
    assert_eq!(
        Parser::for_weekday(today, Thu).weekday(),
        NaiveDate::from_ymd_opt(2022, 12, 29)
    );
    assert_eq!(
        Parser::for_weekday(today, Fri).weekday(),
        NaiveDate::from_ymd_opt(2022, 12, 30)
    );
    assert_eq!(
        Parser::for_weekday(today, Sat).weekday(),
        NaiveDate::from_ymd_opt(2022, 12, 31)
    );
    assert_eq!(
        Parser::for_weekday(today, Sun).weekday(),
        NaiveDate::from_ymd_opt(2023, 1, 1)
    );
}

fn to_string(date: NaiveDate) -> String {
    date.format(FORMAT).to_string()
}

pub fn parse(today: NaiveDate, value: &str) -> Result<String, std::io::Error> {
    let val = value.to_lowercase();
    if let Ok(date) = NaiveDate::parse_from_str(&val, FORMAT) {
        return Ok(to_string(date));
    }

    let parsers: [Parser; 9] = [
        Parser::for_alias(today),
        Parser::for_alias(today),
        Parser::for_weekday(today, Mon),
        Parser::for_weekday(today, Tue),
        Parser::for_weekday(today, Wed),
        Parser::for_weekday(today, Thu),
        Parser::for_weekday(today, Fri),
        Parser::for_weekday(today, Sat),
        Parser::for_weekday(today, Sun),
    ];

    ALIASES
        .iter()
        .position(|&idx| idx == val)
        .map(|idx| &parsers[idx])
        .and_then(|parser| parser.parse(&val))
        .map(to_string)
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
