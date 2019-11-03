// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::user::User;
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_deserialize_user_results() {
    let user_results =
        fs::read_to_string("tests/assets/v2/user.json").expect("Unable to read in JSON file");
    let user: User = serde_json::from_str(&user_results).unwrap();

    assert_eq!(user.application_roles().len(), 0);
    assert_eq!(user.groups().len(), 3);
    assert_eq!(user.timezone, "Australia/Sydney");
    assert!(user.active);
    assert_eq!(user.display_name, "Fred F. User");
    assert_eq!(user.avatar_urls.len(), 4);
    assert_eq!(user.email_address, "fred@example.com");
    assert_eq!(user.name, "fred");
    assert_eq!(
        user.self_link,
        "http://www.example.com/jira/rest/api/2/user?username=fred"
    );
}
