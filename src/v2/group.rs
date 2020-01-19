//! Represents a paginated list of users by group

// ============================================================================
// Use
// ============================================================================
use crate::v2::Pagination;
use crate::v2::User;
use crate::Client;
use crate::Options;
use crate::Response;
use crate::{Deserialize, Serialize};
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
    /// Fetches a paginated list of users inside a given group name. For more
    /// info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/group-getUsersFromGroup
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
            fs::read_to_string("tests/assets/v2/group.json").expect("Unable to read in JSON file");
        let group: Group = serde_json::from_str(&results).unwrap();

        assert_eq!(group.users.len(), 2);
        assert!(!group.pagination.is_last);
        assert_eq!(group.pagination.total, 5);
        assert_eq!(group.pagination.start_at, 3);
        assert_eq!(group.pagination.max_results, 2);
        assert_eq!(group.pagination.next_link, "http://www.example.com/jira/rest/api/2/group/member?groupname=jira-administrators&includeInactiveUsers=false&startAt=4&maxResults=2");
        assert_eq!(
        group.self_link,
        "http://www.example.com/jira/rest/api/2/group/member?groupname=jira-administrators&includeInactiveUsers=false&startAt=2&maxResults=2"
        );
    }
}
