//! An interface for accessing component data from JIRA

// ============================================================================
// Use
// ============================================================================
use crate::client::Client;
use crate::v2::user::User;
use crate::Result;
use serde::{Deserialize, Serialize};

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
    /// Fetches a component by id
    pub fn from_id<T>(c: &Client, id: T) -> Result<Component>
    where
        T: Into<String>,
    {
        let endpoint = format!("/component/{}", id.into());
        c.get("api", "2", &endpoint, None, None)
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
