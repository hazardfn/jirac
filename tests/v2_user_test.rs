// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::User;
use jirac::Client;
use jirac::Credentials;
use mockito::mock;
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_get_user() {
    let result =
        fs::read_to_string("tests/assets/v2/user.json").expect("Unable to read in JSON file");

    let _m = mock("GET", "/rest/api/2/user")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    let c = User::from_username(&client, "user1", &[]).unwrap();

    assert_eq!(c.data.name, "fred");
}
