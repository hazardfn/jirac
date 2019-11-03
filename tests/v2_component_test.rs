// ============================================================================
// External Crates
// ============================================================================
extern crate jirac;

// ============================================================================
// Use
// ============================================================================
use jirac::v2::component::Component;
use jirac::v2::user::User;
use std::fs;

// ============================================================================
// Private
// ============================================================================
fn assert_user(u: User) {
    assert!(!u.active);
    assert_eq!(u.display_name, "Fred F. User");
    assert_eq!(u.avatar_urls.len(), 4);
    assert_eq!(u.name, "fred");
    assert_eq!(
        u.self_link,
        "http://www.example.com/jira/rest/api/2/user?username=fred"
    );
}

// ============================================================================
// Tests
// ============================================================================
#[test]
fn test_deserialize_component_results() {
    let component_results =
        fs::read_to_string("tests/assets/v2/component.json").expect("Unable to read in JSON file");
    let component: Component = serde_json::from_str(&component_results).unwrap();

    assert_eq!(component.project_id, 10000);
    assert_eq!(component.project, "HSP");
    assert!(!component.is_assignee_type_valid);
    assert_user(component.real_assignee.unwrap());
    assert_eq!(component.real_assignee_type, "PROJECT_LEAD");
    assert_user(component.assignee.unwrap());
    assert_eq!(component.assignee_type, "PROJECT_LEAD");
    assert_user(component.lead.unwrap());
    assert_eq!(component.description, "This is a JIRA component");
    assert_eq!(component.name, "Component 1");
    assert_eq!(component.id, "10000");
    assert_eq!(
        component.self_link,
        "http://www.example.com/jira/rest/api/2/component/10000"
    );
}
