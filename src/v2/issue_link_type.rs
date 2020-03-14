//! A representation of JIRA's issue link type format

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
pub struct IssueLinkType {
    /// The link to this issue link type
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The ID of this issue link type
    #[serde(default)]
    pub id: String,

    /// The name of the issue link type
    #[serde(default)]
    pub name: String,

    /// The inward description of this link type
    #[serde(default)]
    pub inward: String,

    /// The outward description of this link type
    #[serde(default)]
    pub outward: String
}

impl IssueLinkType {
    /// Fetches an issue link type given the id of the issue link type. For more info
    /// consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/issueLinkType-getIssueLinkType
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<IssueLinkType>
    where
        I: Into<String>,
    {
        let url = format!("api/2/issueLinkType/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for IssueLinkType {
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
            fs::read_to_string("tests/assets/v2/issue_link_type.json").expect("Unable to read in JSON file");
        let ilt: IssueLinkType = serde_json::from_str(&results).unwrap();

        assert_eq!(ilt.self_link, "http://www.example.com/jira/rest/api/2//issueLinkType/1000");
        assert_eq!(ilt.id, "1000");
        assert_eq!(ilt.name, "Duplicate");
        assert_eq!(ilt.inward, "Duplicated by");
        assert_eq!(ilt.outward, "Duplicates");
    }
}

