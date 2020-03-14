//! Represents a ticket priority in the JIRA system

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
pub struct Priority {
    /// REST API link to the priority
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Status colour for the priority
    #[serde(rename = "statusColor", default)]
    pub status_colour: String,

    /// Priority description
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
}

impl Priority {
    /// Fetches a priority object given its ID. For more info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/priority-getPriority
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<Priority>
    where
        I: Into<String>,
    {
        let url = format!("api/2/priority/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Priority {
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
            fs::read_to_string("tests/assets/v2/priority.json").expect("Unable to read in JSON file");
        let p: Priority = serde_json::from_str(&results).unwrap();

        assert_eq!(p.self_link, "http://www.example.com/jira/rest/api/2/priority/3");
        assert_eq!(p.status_colour, "#009900");
        assert_eq!(p.description, "Major loss of function.");
        assert_eq!(p.icon_url, "http://www.example.com/jira/images/icons/priorities/major.png");
        assert_eq!(p.name, "Major");
    }
}
