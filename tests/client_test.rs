use ezz::client::{Authentication, Zoom};
use ezz::Meeting;
use mockito::{mock, server_url};

fn client(account_id: &str) -> Zoom {
    let auth = Authentication::new(account_id.to_string(), "".to_string(), "".to_string());
    Zoom::from(
        auth,
        format!("{}/token?id=", server_url()),
        format!("{}/meetings", server_url()),
    )
}

fn meeting() -> Meeting {
    Meeting::new(
        "test".to_string(),
        None,
        None,
        "2020-01-01".to_string(),
        "12:00".to_string(),
        60,
    )
}

#[test]
fn test_success_save_meeting() {
    let t = mock("POST", "/token?id=13")
        .with_status(200)
        .with_body(r#"{"access_token":"42"}"#)
        .create();
    let m = mock("POST", "/meetings")
        .with_status(201)
        .with_body(r#"{"join_url":"https://zoom.us/j/123456789"}"#)
        .create();
    let client = client("13");
    let response = client.save(&meeting());
    t.assert();
    m.assert();
    assert!(response.is_ok());
    assert_eq!(
        response.expect("save meeting failed"),
        "https://zoom.us/j/123456789"
    );
}

#[test]
fn test_failed_save_meeting() {
    let client = client("13");
    let statuses: [usize; 3] = [400, 404, 429];
    for status in statuses {
        let t = mock("POST", "/token?id=13")
            .with_status(200)
            .with_body(r#"{"access_token":"42"}"#)
            .create();
        let m = mock("POST", "/meetings").with_status(status).create();
        let response = client.save(&meeting());
        t.assert();
        m.assert();
        assert!(response.is_err());
    }
}

#[test]
fn test_failed_token_on_save_meeting() {
    let t = mock("POST", "/token?id=13").with_status(404).create();
    let m = mock("POST", "/meetings")
        .with_status(201)
        .with_body(r#"{"join_url":"https://zoom.us/j/123456789"}"#)
        .expect(0)
        .create();
    let client = client("13");
    let response = client.save(&meeting());
    t.assert();
    m.assert();
    assert!(response.is_err());
}
