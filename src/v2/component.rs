//! An interface for accessing component data from JIRA

// ============================================================================
// Use
// ============================================================================
use crate::v2::User;
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    /// REST API link to component
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// ID of the component
    #[serde(default)]
    pub id: String,

    /// Name of the component
    #[serde(default)]
    pub name: String,

    /// Description of the component
    #[serde(default)]
    pub description: String,

    /// User that is the lead of the component
    #[serde(default)]
    pub lead: Option<User>,

    /// Assignee type
    #[serde(rename = "assigneeType", default)]
    pub assignee_type: String,

    /// User that is assigned to the component
    #[serde(default)]
    pub assignee: Option<User>,

    /// Real assignee type
    #[serde(rename = "realAssigneeType", default)]
    pub real_assignee_type: String,

    /// User that is assigned to the component
    #[serde(rename = "realAssignee", default)]
    pub real_assignee: Option<User>,

    /// Is the assignee type valid
    #[serde(rename = "isAssigneeTypeValid", default)]
    pub is_assignee_type_valid: bool,

    /// Project key the component is assigned to
    #[serde(default)]
    pub project: String,

    /// The id of the project the component is assigned to
    #[serde(rename = "projectId", default)]
    pub project_id: usize,
}

impl Component {
    /// Fetches a single component by id, for more information consult the api
    /// docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/component-getComponent
    pub fn from_id<T>(c: &Client, id: T) -> Response<Self>
    where
        T: Into<String>,
    {
        let endpoint = format!("api/2/component/{}", id.into());
        c.get(&endpoint)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Component {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_deserialize_results() {
        let results = fs::read_to_string("tests/assets/v2/component.json")
            .expect("Unable to read in JSON file");
        let component: Component = serde_json::from_str(&results).unwrap();

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
}
