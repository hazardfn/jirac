//! Represents a group a user belongs to

// ============================================================================
// Use
// ============================================================================
use crate::client::Client;
use crate::options::Options;
use crate::v2::pagination::Pagination;
use crate::v2::user::User;
use crate::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Public Structures
// ============================================================================
pub struct GroupOptions {
    /// If true a get request will return inactive users in the list
    include_inactive_users: bool,
}

impl Options for GroupOptions {
    fn to_query(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(
            String::from("includeInactiveUsers"),
            self.include_inactive_users.to_string(),
        );
        h
    }
}

impl Default for GroupOptions {
    fn default() -> Self {
        GroupOptions {
            include_inactive_users: false,
        }
    }
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
    ) -> Response<Group>
    where
        G: Into<String>,
    {
        let mut c = c.clone();
        let mut query =
            Client::unpack_options(vec![&opts.unwrap_or_default(), &page.unwrap_or_default()]);

        query.insert("groupname".to_string(), name.into());

        c = c.add_query(query);
        c.get("api/2/group/member")
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
