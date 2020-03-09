//! A representation of JIRA's issue type format

// ============================================================================
// Use
// ============================================================================
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct IssueType {
    /// The link to this issue type
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The ID of this issue type
    #[serde(default)]
    pub id: String,

    /// A description of the issue type
    #[serde(default)]
    pub description: String,

    /// URL to the issue type's icon
    #[serde(rename = "iconUrl", default)]
    pub icon_url: String,

    /// Name of the issue type
    #[serde(default)]
    pub name: String,

    /// Is the issue a subtask?
    #[serde(default)]
    pub subtask: bool,

    /// Avatar id
    #[serde(rename = "avatarId", default)]
    pub avatar_id: i64
}

impl IssueType {
    /// Fetches an issue type given the id of the issue type. For more info
    /// consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/issuetype-getIssueType
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<IssueType>
    where
        I: Into<String>,
    {
        let url = format!("api/2/issuetype/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for IssueType {
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
            fs::read_to_string("tests/assets/v2/issuetype.json").expect("Unable to read in JSON file");
        let it: IssueType = serde_json::from_str(&results).unwrap();

        assert_eq!(it.self_link, "http://localhost:8080/rest/api/2/issuetype/10003");
        assert_eq!(it.id, "10003");
        assert_eq!(it.description, "A task that needs to be done.");
        assert_eq!(it.icon_url, "http://localhost:8080/secure/viewavatar?size=xsmall&avatarId=10318&avatarType=issuetype");
        assert_eq!(it.name, "Task");
        assert_eq!(it.subtask, false);
        assert_eq!(it.avatar_id, 10318);   
    }
}

