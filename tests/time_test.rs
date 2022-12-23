use ezz::time::parse;

#[test]
fn test_parse_time() {
    assert_eq!(parse("08:00").unwrap(), "08:00");
    assert_eq!(parse("8:05").unwrap(), "08:05");
    assert_eq!(parse("16:10").unwrap(), "16:10");
    assert_eq!(parse("08h15").unwrap(), "08:15");
    assert_eq!(parse("8h20").unwrap(), "08:20");
    assert_eq!(parse("16h25").unwrap(), "16:25");
    assert_eq!(parse("08h").unwrap(), "08:00");
    assert_eq!(parse("8h").unwrap(), "08:00");
    assert_eq!(parse("16h").unwrap(), "16:00");
    assert_eq!(parse("08H30").unwrap(), "08:30");
    assert_eq!(parse("8H35").unwrap(), "08:35");
    assert_eq!(parse("16H40").unwrap(), "16:40");
    assert_eq!(parse("08H").unwrap(), "08:00");
    assert_eq!(parse("8H").unwrap(), "08:00");
    assert_eq!(parse("16H").unwrap(), "16:00");
    assert!(parse("24h00").is_err());
    assert!(parse("0:60").is_err());
    assert!(parse("ezz").is_err());
}
