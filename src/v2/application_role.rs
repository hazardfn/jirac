//! Provides REST access to JIRA's Application Roles.

// ============================================================================
// Use
// ============================================================================
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
pub struct ApplicationRoleOptions {
    /// The hash of the version to update. Optional Param.
    ///
    /// Optional: If versionHash is passed through the If-Match header the
    /// request will be rejected if not the same as server
    if_match: String,
}

impl ApplicationRoleOptions {
    pub fn new(if_match: String) -> Self {
        ApplicationRoleOptions { if_match }
    }
}

#[readonly::make]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationRole {
    /// Identifier of the application role
    #[readonly]
    #[serde(default)]
    pub key: String,

    /// Name of the application role
    #[readonly]
    #[serde(default)]
    pub name: String,

    /// List of groups associated with the role
    #[serde(default)]
    pub groups: Vec<String>,

    /// List of default groups for the role
    #[serde(rename = "defaultGroups", default)]
    pub default_groups: Vec<String>,

    /// Selected by default
    #[readonly]
    #[serde(rename = "selectedByDefault", default)]
    pub selected_by_default: bool,

    /// Defined
    #[readonly]
    #[serde(default)]
    pub defined: bool,

    /// Number of seats
    #[readonly]
    #[serde(rename = "numberOfSeats", default)]
    pub number_of_seats: usize,

    /// Remaining seats
    #[readonly]
    #[serde(rename = "remainingSeats", default)]
    pub remaining_seats: usize,

    /// User count
    #[readonly]
    #[serde(rename = "userCount", default)]
    pub user_count: usize,

    /// User count description
    #[readonly]
    #[serde(rename = "userCountDescription", default)]
    pub user_count_description: String,

    /// Has unlimited seats
    #[readonly]
    #[serde(rename = "hasUnlimitedSeats", default)]
    pub has_unlimited_seats: bool,

    /// Platform
    #[readonly]
    #[serde(default)]
    pub platform: bool,
}

impl ApplicationRole {
    /// Fetches a single role by key. For more information see the atlassian docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-get
    pub fn from_key<K>(c: &Client, key: K) -> Response<Self>
    where
        K: Into<String>,
    {
        let endpoint = format!("api/2/applicationrole/{}", key.into());
        c.get(&endpoint)
    }

    /// Fetches all available roles. For more information see the atlassian
    /// docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-getAll
    pub fn all(c: &Client) -> Response<Vec<Self>> {
        c.get("api/2/applicationrole")
    }

    /// Will bulk update roles given a vector of ApplicationRole. For more
    /// detailed information see `update` below or consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-putBulk
    pub fn update_bulk(
        c: &Client,
        a: Vec<Self>,
        o: Option<ApplicationRoleOptions>,
    ) -> Response<Vec<Self>> {
        let mut c = c.clone();

        if let Some(o) = o {
            c = c.add_header("If-Match", o.if_match);
        }

        c.put("api/2/applicationrole", a)
    }

    /// Updates the role with the information currently in the struct. Note
    /// that only certain fields can be updated here as per the API spec, the
    /// others are silently ignored. See docs for more info:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-put
    pub fn update(&self, c: &Client, o: Option<ApplicationRoleOptions>) -> Response<Self> {
        let mut c = c.clone();
        let endpoint = format!("api/2/applicationrole/{}", self.key);

        if let Some(o) = o {
            c = c.add_header("If-Match", o.if_match);
        }

        c.put(&endpoint, self)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for ApplicationRole {
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
        let results = fs::read_to_string("tests/assets/v2/application_role.json")
            .expect("Unable to read in JSON file");
        let application_role: ApplicationRole = serde_json::from_str(&results).unwrap();

        assert!(!application_role.platform);
        assert!(!application_role.has_unlimited_seats);
        assert_eq!(application_role.user_count_description, "5 developers");
        assert_eq!(application_role.user_count, 5);
        assert_eq!(application_role.remaining_seats, 5);
        assert_eq!(application_role.number_of_seats, 10);
        assert!(!application_role.defined);
        assert!(!application_role.selected_by_default);
        assert_eq!(application_role.default_groups.len(), 1);
        assert_eq!(application_role.name, "JIRA Software");
        assert_eq!(application_role.groups.len(), 2);
        assert_eq!(application_role.key, "jira-software");
    }
}
