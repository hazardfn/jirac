// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::group::Group;
use std::fs;

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_deserialize_group_results() {
    let group_results =
        fs::read_to_string("tests/assets/v2/group.json").expect("Unable to read in JSON file");
    let group: Group = serde_json::from_str(&group_results).unwrap();

    assert_eq!(group.users.len(), 2);
    assert!(!group.pagination.is_last);
    assert_eq!(group.pagination.total, 5);
    assert_eq!(group.pagination.start_at, 3);
    assert_eq!(group.pagination.max_results, 2);
    assert_eq!(group.pagination.next_link, "http://www.example.com/jira/rest/api/2/group/member?groupname=jira-administrators&includeInactiveUsers=false&startAt=4&maxResults=2");
    assert_eq!(
        group.self_link,
        "http://www.example.com/jira/rest/api/2/group/member?groupname=jira-administrators&includeInactiveUsers=false&startAt=2&maxResults=2"
    );
}
