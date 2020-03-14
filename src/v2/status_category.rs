//! Represents a status category in the JIRA system

// ============================================================================
// Use
// ============================================================================
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCategory {
    /// REST API link to the status category
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// ID of the status category
    #[serde(default)]
    pub id: i64,

    /// Staus category key
    #[serde(default)]
    pub key: String,

    /// Name of the colour given to the status category
    #[serde(rename = "colorName", default)]
    pub colour_name: String,

    /// Name of the status category
    #[serde(default)]
    pub name: String,

}

impl StatusCategory {
    /// Fetches a status category given its ID or Key. For more info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/statuscategory-getStatusCategory
    pub fn from_id_or_key<I>(
        c: &Client,
        id: I,
    ) -> Response<StatusCategory>
    where
        I: Into<String>,
    {
        let url = format!("api/2/statuscategory/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for StatusCategory {
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
            fs::read_to_string("tests/assets/v2/status_category.json").expect("Unable to read in JSON file");
        let s: StatusCategory = serde_json::from_str(&results).unwrap();

        assert_eq!(s.self_link, "http://localhost:8090/jira/rest/api/2.0/statuscategory/1");
        assert_eq!(s.id, 1);
        assert_eq!(s.key, "in-flight");
        assert_eq!(s.colour_name, "yellow");
        assert_eq!(s.name, "In Progress");
    }
}
