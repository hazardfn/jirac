//! Represents a paginated list of users by group

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
pub struct Resolution {
    /// REST API link to group
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// ID of the resolution
    #[serde(default)]
    pub id: String,

    /// Description of the resolution
    #[serde(default)]
    pub description: String,

    /// Name of the resolution
    #[serde(default)]
    pub name: String,
}

impl Resolution {
    /// Fetches a resolution given the id of the resolution. For more info
    /// consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/resolution-getResolution
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<Resolution>
    where
        I: Into<String>,
    {
        let url = format!("api/2/resolution/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Resolution {
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
            fs::read_to_string("tests/assets/v2/resolution.json").expect("Unable to read in JSON file");
        let res: Resolution = serde_json::from_str(&results).unwrap();

        assert_eq!(res.self_link, "http://localhost:8080/rest/api/2/resolution/10000");
        assert_eq!(res.id, "10000");
        assert_eq!(res.description, "Work has been completed on this issue.");
        assert_eq!(res.name, "Done");
        
    }
}
