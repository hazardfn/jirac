//! Represents an attachment in the JIRA system

// ============================================================================
// Use
// ============================================================================
use crate::v2::StatusCategory;
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    /// REST API link to the status
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Description of the status
    #[serde(default)]
    pub description: String,

    /// URL to icon of the status
    #[serde(rename = "iconUrl", default)]
    pub icon_url: String,

    /// Name of the status
    #[serde(default)]
    pub name: String,

    /// ID of the status
    #[serde(default)]
    pub id: String,

    /// Status category of the status
    #[serde(rename = "statusCategory", default)]
    pub status_category: Option<StatusCategory>
}

impl Status {
    /// Fetches a status given its ID or name. For more info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/status-getStatus
    pub fn from_id_or_name<I>(
        c: &Client,
        id: I,
    ) -> Response<Status>
    where
        I: Into<String>,
    {
        let url = format!("api/2/status/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Status {
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
        let results =
            fs::read_to_string("tests/assets/v2/status.json").expect("Unable to read in JSON file");
        let s: Status = serde_json::from_str(&results).unwrap();

        assert_eq!(s.self_link, "http://localhost:8090/jira/rest/api/2.0/status/10000");
        assert_eq!(s.description, "The issue is currently being worked on.");
        assert_eq!(s.icon_url, "http://localhost:8090/jira/images/icons/progress.gif");
        assert_eq!(s.name, "In Progress");
        assert_eq!(s.id, "10000");
        assert_eq!(s.status_category.unwrap().id, 1);
    }
}
