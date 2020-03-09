// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::Group;
use jirac::Client;
use jirac::Credentials;
use mockito::{mock, Matcher};
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_get_member() {
    let result =
        fs::read_to_string("tests/assets/v2/group.json").expect("Unable to read in JSON file");

    let _m = mock("GET", "/rest/api/2/group/member")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("includeInactiveUsers".into(), "false".into()),
            Matcher::UrlEncoded("maxResults".into(), "50".into()),
            Matcher::UrlEncoded("groupname".into(), "group1".into()),
            Matcher::UrlEncoded("startAt".into(), "0".into())
          ]))
        .with_body(result)
        .create();

    let url = &mockito::server_url();
    let creds = Credentials::new_basic("test", "test").unwrap();
    let client = Client::new(url, creds);

    let c = Group::from_name(&client, "group1", None, None).unwrap();

    assert_eq!(c.data.users.len(), 2);
}
