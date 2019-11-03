//! Represents a group a user belongs to

// ============================================================================
// Use
// ============================================================================
use crate::client::Client;
use crate::v2::pagination::Pagination;
use crate::v2::user::User;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Public Structures
// ============================================================================
pub struct GroupOptions {
    /// If true a get request will return inactive users in the list
    include_inactive_users: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    /// REST API link to group
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Pagination values
    #[serde(flatten, default)]
    pub pagination: Pagination,

    /// List of users in group
    #[serde(rename = "values", default)]
    pub users: Vec<User>,

    /// Name of the group
    #[serde(default)]
    pub name: String,
}

impl Group {
    /// Fetches a user by username
    pub fn from_name<G>(
        c: &Client,
        name: G,
        opts: Option<GroupOptions>,
        page: Option<Pagination>,
    ) -> Result<Group>
    where
        G: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("groupname".to_string(), name.into());

        if let Some(o) = opts {
            query.insert(
                "includeInactiveUsers".to_string(),
                o.include_inactive_users.to_string(),
            );
        }

        if let Some(p) = page {
            query.insert("startAt".to_string(), p.start_at.to_string());
            query.insert("maxResults".to_string(), p.max_results.to_string());
        }

        c.get("api", "2", "group/member", Some(query), None)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Group {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
