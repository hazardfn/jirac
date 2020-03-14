//! A representation of JIRA's issue link format

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Issue, IssueLinkType};
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct IssueLink {
    /// The link to this issue link 
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The ID of this issue link
    #[serde(default)]
    pub id: String,

    /// The issue link type this link represents (Blocked by, Related to etc.)
    #[serde(rename = "type", default)]
    pub issue_link_type: Option<IssueLinkType>,

    /// The outward issue connected to this link.
    #[serde(rename = "outwardIssue", default)]
    pub outward_issue: Option<Issue>,

    /// The inward issue connected to this link.
    #[serde(rename = "inwardIssue", default)]
    pub inward_issue: Option<Issue>
}

impl IssueLink {
    /// Fetches an issue link given the id of the issue link. For more info
    /// consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/issueLink-getIssueLink
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<IssueLink>
    where
        I: Into<String>,
    {
        let url = format!("api/2/issueLink/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for IssueLink {
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
            fs::read_to_string("tests/assets/v2/issue_link.json").expect("Unable to read in JSON file");
        let il: IssueLink = serde_json::from_str(&results).unwrap();

        assert_eq!(il.self_link, "http://localhost:8080/rest/api/2/issueLink/10101");
        assert_eq!(il.id, "10101");
        assert_eq!(il.issue_link_type.unwrap().name, "Blocks");
        assert_eq!(il.inward_issue.unwrap().key, "TEST-1");
        assert_eq!(il.outward_issue.unwrap().key, "TEST-9");
    }
}

