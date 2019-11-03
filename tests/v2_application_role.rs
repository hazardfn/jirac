// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::application_role::ApplicationRole;
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_deserialize_application_role_results() {
    let application_role_results = fs::read_to_string("tests/assets/v2/application_role.json")
        .expect("Unable to read in JSON file");
    let application_role: ApplicationRole =
        serde_json::from_str(&application_role_results).unwrap();

    assert!(!application_role.platform);
    assert!(!application_role.has_unlimited_seats);
    assert_eq!(application_role.user_count_description, "5 developers");
    assert_eq!(application_role.user_count, 5);
    assert_eq!(application_role.remaining_seats, 5);
    assert_eq!(application_role.number_of_seats, 10);
    assert!(!application_role.defined);
    assert!(!application_role.selected_by_default);
    assert_eq!(application_role.default_groups.len(), 1);
    assert_eq!(application_role.name, "JIRA Software");
    assert_eq!(application_role.groups.len(), 2);
    assert_eq!(application_role.key, "jira-software");
}
