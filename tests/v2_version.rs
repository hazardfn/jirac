// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::Version;
use jirac::Client;
use jirac::Credentials;
use mockito::mock;
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_get() {
    let result =
        fs::read_to_string("tests/assets/v2/version.json").expect("Unable to read in JSON file");

    let _m = mock("GET", "/rest/api/2/version/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    let c = Version::from_id(&client, "1").unwrap();

    assert_eq!(c.data.project_id, 10000);
}

#[test]
fn test_create() {
    let result =
        fs::read_to_string("tests/assets/v2/version.json").expect("Unable to read in JSON file");

    let _m = mock("POST", "/rest/api/2/version")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(&result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);
    let version: Version = serde_json::from_str(&result).unwrap();

    let c = version.create(&client).unwrap();

    assert_eq!(c.data.project_id, 10000);
}
