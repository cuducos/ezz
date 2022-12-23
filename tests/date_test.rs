use chrono::NaiveDate;
use ezz::date::parse;

#[test]
fn test_parse_date() {
    let today = NaiveDate::from_ymd_opt(2022, 12, 28).unwrap();
    assert_eq!(parse(today, "2023-02-08").unwrap(), "2023-02-08");
    assert_eq!(parse(today, "today").unwrap(), "2022-12-28");
    assert_eq!(parse(today, "tomorrow").unwrap(), "2022-12-29");
    assert_eq!(parse(today, "monday").unwrap(), "2023-01-02");
    assert_eq!(parse(today, "tuesday").unwrap(), "2023-01-03");
    assert_eq!(parse(today, "Wednesday").unwrap(), "2022-12-28");
    assert_eq!(parse(today, "thursday").unwrap(), "2022-12-29");
    assert_eq!(parse(today, "friday").unwrap(), "2022-12-30");
    assert_eq!(parse(today, "saturday").unwrap(), "2022-12-31");
    assert_eq!(parse(today, "sunday").unwrap(), "2023-01-01");
    assert!(parse(today, "ezz").is_err());
}
