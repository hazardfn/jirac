//! Provides REST access to JIRA's Application Roles.

// ============================================================================
// Use
// ============================================================================
use crate::client::Client;
use crate::reqwest::header::{HeaderMap, HeaderValue};
use crate::Result;
use serde::{Deserialize, Serialize};

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
    pub fn from_key<K>(c: &Client, key: K) -> Result<Self>
    where
        K: Into<String>,
    {
        let endpoint = format!("applicationrole/{}", key.into());
        c.get("api", "2", &endpoint, None, None)
    }

    /// Fetches all available roles. For more information see the atlassian
    /// docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-getAll
    pub fn all(c: &Client) -> Result<Vec<Self>> {
        c.get("api", "2", "applicationrole", None, None)
    }

    /// Will bulk update roles given a vector of ApplicationRole. For more
    /// detailed information see `update` below or consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-putBulk
    pub fn update_bulk(
        c: &Client,
        a: Vec<Self>,
        o: Option<ApplicationRoleOptions>,
    ) -> Result<Vec<Self>> {
        let headers = if let Some(o) = o {
            let mut headers = HeaderMap::new();
            headers.insert("If-Match", HeaderValue::from_str(&o.if_match).unwrap());
            Some(headers)
        } else {
            None
        };

        c.put("api", "2", "applicationrole", None, headers, a)
    }

    /// Updates the role with the information currently in the struct. Note
    /// that only certain fields can be updated here as per the API spec, the
    /// others are silently ignored. See docs for more info:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/7.6.1/#api/2/applicationrole-put
    pub fn update(&self, c: &Client, o: Option<ApplicationRoleOptions>) -> Result<Self> {
        let endpoint = format!("applicationrole/{}", self.key);
        let headers = if let Some(o) = o {
            let mut headers = HeaderMap::new();
            headers.insert("If-Match", HeaderValue::from_str(&o.if_match).unwrap());
            Some(headers)
        } else {
            None
        };
        c.put("api", "2", &endpoint, None, headers, self)
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
