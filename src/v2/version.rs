//! An interface for accessing versions (fixVersion) from JIRA

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
pub struct Version {
    /// Internal id representation of the version
    #[serde(default)]
    pub id: String,

    /// A link to the JSON API version object
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The description of the version
    #[serde(default)]
    pub description: String,

    /// Name of the version
    #[serde(default)]
    pub name: String,

    /// Release date is the ACTUAL release date your software was released.
    /// You should only really set this field when the release is complete.
    #[serde(rename = "releaseDate", default)]
    pub release_date: String,

    /// User release date is the proposed date of release given on version
    /// creation. This field and release_date will hold the same value once a
    /// release is completed.
    ///
    /// Unless you are a victim of the following bug:
    /// https://jira.atlassian.com/browse/JRACLOUD-32389?_ga=2.109234573.942351204.1572973048-487013296.1572973048
    #[serde(rename = "userReleaseDate", default)]
    pub user_release_date: String,

    /// Returns true if version is archived
    #[serde(default)]
    pub archived: bool,

    /// Returns true if the version has been marked as released
    #[serde(default)]
    pub released: bool,

    /// Is release overdue
    #[serde(default)]
    pub overdue: bool,

    /// The id of the project this version belongs to
    #[serde(rename = "projectId", default)]
    pub project_id: i64,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            id: String::new(),
            self_link: String::new(),
            description: String::new(),
            name: String::new(),
            release_date: String::new(),
            user_release_date: String::new(),
            archived: false,
            released: false,
            overdue: false,
            project_id: 0,
        }
    }
}

impl Version {
    /// Returns a blank version
    pub fn new() -> Self {
        Version::default()
    }

    /// Fetches a version by id
    pub fn from_id<I>(c: &Client, id: I) -> Response<Version>
    where
        I: Into<String>,
    {
        let endpoint = format!("api/2/version/{}", id.into());
        c.get(&endpoint)
    }

    /// Updates a version
    pub fn update(self, c: &Client) -> Response<Version> {
        let endpoint = format!("api/v2/version/{}", &self.id);
        c.put(&endpoint, self)
    }

    /// Creates a new version
    pub fn create(self, c: &Client) -> Response<Version> {
        c.post("api/2/version", self)
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
        let results = fs::read_to_string("tests/assets/v2/version.json")
            .expect("Unable to read in JSON file");
        let version: Version = serde_json::from_str(&results).unwrap();
        assert_eq!(version.project_id, 10000);
        assert_eq!(version.user_release_date, "6/Jul/2010");
        assert!(version.overdue);
        assert_eq!(version.release_date, "2010-07-06");
        assert!(version.released);
        assert!(!version.archived);
        assert_eq!(version.name, "New Version 1");
        assert_eq!(version.description, "An excellent version");
        assert_eq!(version.id, "10000");
        assert_eq!(
            version.self_link,
            "http://www.example.com/jira/rest/api/2/version/10000"
        );
    }
}
