//! Interface for users in JIRA

// ============================================================================
// Use
// ============================================================================
use crate::v2::{ApplicationRole, Group, Item, Pagination};
use crate::Client;
use crate::Options;
use crate::Response;
use crate::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

// ============================================================================
// Public Enums
// ============================================================================
pub enum UserExpand {
    Groups,
    ApplicationRoles,
}

impl UserExpand {
    pub fn to_string(&self) -> &str {
        match &self {
            UserExpand::Groups => "groups",
            UserExpand::ApplicationRoles => "applicationRoles",
        }
    }
}

// ============================================================================
// Public Structures
// ============================================================================
pub struct UserOptions {
    /// Include inactive users in requests.
    include_inactive: bool,

    /// Include active users in requests.
    include_active: bool,
}

impl Options for UserOptions {
    fn to_query(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(
            String::from("includeInactive"),
            self.include_inactive.to_string(),
        );
        h.insert(
            String::from("includeActive"),
            self.include_active.to_string(),
        );
        h
    }
}

impl UserOptions {
    pub fn new(include_active: bool, include_inactive: bool) -> Self {
        UserOptions {
            include_active,
            include_inactive,
        }
    }
}

impl Default for UserOptions {
    fn default() -> Self {
        UserOptions {
            include_active: true,
            include_inactive: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Returns true if the user is active in Jira
    #[serde(default)]
    pub active: bool,

    /// Returns avatar urls for the user
    #[serde(rename = "avatarUrls", default)]
    pub avatar_urls: BTreeMap<String, String>,

    /// Returns the display name of the user
    #[serde(rename = "displayName", default)]
    pub display_name: String,

    /// The email address associated with the account
    #[serde(rename = "emailAddress", default)]
    pub email_address: String,

    /// The id key associated with the user
    #[serde(default)]
    pub key: String,

    /// The username of the user
    #[serde(default)]
    pub name: String,

    /// A link to the user object
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The timezone set for the user account
    #[serde(rename = "timeZone", default)]
    pub timezone: String,

    /// List of groups the user belongs to
    #[serde()]
    pub groups: Option<Item>,

    /// List of application roles the user has
    #[serde(rename = "applicationRoles")]
    pub application_roles: Option<Item>,
}

impl User {
    pub fn search<S>(
        c: &Client,
        search: S,
        opts: Option<UserOptions>,
        page: Option<Pagination>,
    ) -> Response<Vec<User>>
    where
        S: Into<String>,
    {
        let mut query =
            Client::unpack_options(vec![&opts.unwrap_or_default(), &page.unwrap_or_default()]);

        query.insert("username".to_string(), search.into());

        c.clone().add_query(query).get("api/2/user/search")
    }

    /// Fetches a user by username
    pub fn from_username<U>(c: &Client, username: U, expand: Vec<UserExpand>) -> Response<User>
    where
        U: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();

        query.insert("username".to_string(), username.into());
        query.extend(expand_to_hashmap(expand));

        c.clone().add_query(query).get("api/2/user")
    }

    /// Fetches a user by key
    pub fn from_key<K>(c: &Client, key: K, expand: Vec<UserExpand>) -> Response<User>
    where
        K: Into<String>,
    {
        let mut query: HashMap<String, String> = HashMap::new();

        query.insert("key".to_string(), key.into());
        query.extend(expand_to_hashmap(expand));

        c.clone().add_query(query).get("api/2/user")
    }

    pub fn groups(&self) -> Vec<Group> {
        if let Some(i) = &self.groups {
            serde_json::value::from_value(i.items.clone()).unwrap()
        } else {
            Vec::new()
        }
    }

    pub fn application_roles(&self) -> Vec<ApplicationRole> {
        if let Some(i) = &self.application_roles {
            serde_json::value::from_value(i.items.clone()).unwrap()
        } else {
            Vec::new()
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for User {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// ============================================================================
// Private
// ============================================================================
fn expand_to_hashmap(e: Vec<UserExpand>) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();
    let mut value = e.iter().fold(String::from(""), |acc, e| {
        format!("{}{},", acc, e.to_string())
    });

    value.pop();

    res.insert("expand".to_string(), value);
    res
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_expand_to_hashmap() {
        let e = vec![UserExpand::ApplicationRoles, UserExpand::Groups];
        let h = expand_to_hashmap(e);

        assert!(h.get("expand").unwrap().contains("groups"));
        assert!(h.get("expand").unwrap().contains("applicationRoles"));
    }

    #[test]
    fn test_deserialize_results() {
        let results =
            fs::read_to_string("tests/assets/v2/user.json").expect("Unable to read in JSON file");
        let user: User = serde_json::from_str(&results).unwrap();
        assert_eq!(user.application_roles().len(), 0);
        assert_eq!(user.groups().len(), 3);
        assert_eq!(user.timezone, "Australia/Sydney");
        assert!(user.active);
        assert_eq!(user.display_name, "Fred F. User");
        assert_eq!(user.avatar_urls.len(), 4);
        assert_eq!(user.email_address, "fred@example.com");
        assert_eq!(user.name, "fred");
        assert_eq!(
            user.self_link,
            "http://www.example.com/jira/rest/api/2/user?username=fred"
        );
    }
}
