// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;
extern crate mockito;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::{ApplicationRole, ApplicationRoleOptions};
use jirac::Client;
use jirac::Credentials;
use jirac::Error;
use mockito::mock;
use std::fs;

// ============================================================================
// Tests (Happy Path)
// ============================================================================
#[test]
fn test_get_from_key() {
    let result = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");

    let _m = mock("GET", "/rest/api/2/applicationrole/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    let a = ApplicationRole::from_key(&client, "1").unwrap();

    assert_eq!(a.data.key, "jira-software");
}

#[test]
fn test_get() {
    let result = fs::read_to_string("tests/assets/v2/application_role_all.json")
        .expect("Unable to read in JSON file");

    let _m = mock("GET", "/rest/api/2/applicationrole")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    let a = ApplicationRole::all(&client).unwrap();

    assert_eq!(a.data.len(), 2);
}

#[test]
fn test_put_with_key() {
    let result = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");

    let a: ApplicationRole = serde_json::from_str(&result).unwrap();

    let _m = mock("PUT", "/rest/api/2/applicationrole/jira-software")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    assert_eq!(a.update(&client, None).unwrap().data.key, "jira-software");
}

#[test]
fn test_put_with_key_and_options() {
    let result = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");

    let a: ApplicationRole = serde_json::from_str(&result).unwrap();
    let o = ApplicationRoleOptions::new(String::from("test"));

    let _m = mock("PUT", "/rest/api/2/applicationrole/jira-software")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("If-Match", "test")
        .with_body(&result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    assert_eq!(
        a.update(&client, Some(o)).unwrap().headers["If-Match"],
        "test"
    );
}

#[test]
fn test_put_bulk() {
    let result = fs::read_to_string("tests/assets/v2/application_role_all.json")
        .expect("Unable to read in JSON file");

    let a: Vec<ApplicationRole> = serde_json::from_str(&result).unwrap();

    let _m = mock("PUT", "/rest/api/2/applicationrole")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    assert_eq!(
        ApplicationRole::update_bulk(&client, a, None)
            .unwrap()
            .data
            .len(),
        2
    );
}

// ============================================================================
// Tests (Sad Path)
// ============================================================================
#[test]
fn test_put_with_key_unauthorized() {
    let result = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");

    let a: ApplicationRole = serde_json::from_str(&result).unwrap();

    let _m = mock("PUT", "/rest/api/2/applicationrole/jira-software")
        .with_status(401)
        .with_header("content-type", "application/json")
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    match a.update(&client, None) {
        Err(Error::Unauthorized) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_put_with_key_precondition_failed() {
    let result = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");

    let a: ApplicationRole = serde_json::from_str(&result).unwrap();

    let _m = mock("PUT", "/rest/api/2/applicationrole/jira-software")
        .with_status(412)
        .with_header("content-type", "application/json")
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new("test", "test").unwrap();
    let client = Client::new(url, creds);

    match a.update(&client, None) {
        Err(Error::PreconditionFailed) => assert!(true),
        _ => assert!(false),
    };
}
