//! Represents permissions in the JIRA system

// ============================================================================
// Use
// ============================================================================
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};
use crate::QueryOptions;
use std::collections::BTreeMap;

// ============================================================================
// Public Enums
// ============================================================================
pub enum MyPermissionKey {
    ProjectKey,
    ProjectId,
    IssueKey,
    IssueId
}

impl QueryOptions for MyPermissionKey {
    fn to_string(&self) -> String {
        match &self {
            MyPermissionKey::IssueId => "issueId".to_string(),
            MyPermissionKey::IssueKey => "issueKey".to_string(),
            MyPermissionKey::ProjectId => "projectId".to_string(),
            MyPermissionKey::ProjectKey => "projectKey".to_string()
        }
    }
}

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Internal representation of the permission
    #[serde(default)]
    pub key: String,

    /// Human readable name of the permission.
    #[serde(default)]
    pub name: String,

    /// Type of permission
    #[serde(rename = "type", default)]
    pub permission_type: String,

    /// Description of the permission
    #[serde(default)]
    pub description: String,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCollection {
    /// Collection of permissions
    pub permissions: BTreeMap<String, Permission>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyPermission {
    /// ID of the permission
    #[serde(default)]
    pub id: String,

    /// Internal representation of the permission
    #[serde(default)]
    pub key: String,

    /// Human readable name of the permission.
    #[serde(default)]
    pub name: String,

    /// Description of the permission
    #[serde(default)]
    pub description: String,

    /// Do you have the permission?
    #[serde(rename = "havePermission", default)]
    pub have_permission: bool,

    /// Is the permission key deprecated?
    #[serde(default)]
    pub deprecated_key: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyPermissionCollection {
    /// Collection of permissions
    pub permissions: BTreeMap<String, MyPermission>
}

impl MyPermission {
    /// Fetches permissions for the given permission key using the currently 
    /// authenticated credentials, see the docs for more details:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2-getPermissions
    pub fn my_permissions_for_key<P>(
        c: &Client,
        key: MyPermissionKey,
        value: P
    ) -> Response<MyPermissionCollection>
    where
        P: Into<String>,
    {
        let url = format!("/api/2/mypermissions?{}={}", key.to_string(), value.into());

        c.get(&url)
    }
}

impl Permission {
    /// Fetches all permissions within the JIRA instance, see the docs for more 
    /// details:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2-getAllPermissions
    pub fn all_permissions(
        c: &Client
    ) -> Response<PermissionCollection>
    {
        let url = "/api/2/permissions";

        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Permission {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for PermissionCollection {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for MyPermission {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for MyPermissionCollection {
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
    fn test_deserialize_mypermission_results() {
        let results =
            fs::read_to_string("tests/assets/v2/my_permissions.json").expect("Unable to read in JSON file");
        let p: MyPermissionCollection = serde_json::from_str(&results).unwrap();

        assert_eq!(p.permissions.len(), 70);
        assert_eq!(p.permissions.get("CREATE_ISSUES").unwrap().id, "11");
    }

    #[test]
    fn test_deserialize_permission_results() {
        let results =
            fs::read_to_string("tests/assets/v2/permissions.json").expect("Unable to read in JSON file");
        let p: PermissionCollection = serde_json::from_str(&results).unwrap();

        assert_eq!(p.permissions.len(), 42);
        assert_eq!(p.permissions.get("CREATE_ISSUES").unwrap().permission_type, "PROJECT");
    }

}
