//! Represents a project in Jira

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Component, IssueType, User, Version};
use crate::Client;
use crate::QueryOptions;
use crate::Response;
use crate::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

// ============================================================================
// Public Enums
// ============================================================================
pub enum ProjectExpand {
    Description,
    Lead,
    Url,
    ProjectKeys,
}

impl QueryOptions for ProjectExpand {
    fn to_string(&self) -> String {
        match &self {
            ProjectExpand::Description => "description".to_string(),
            ProjectExpand::Lead => "lead".to_string(),
            ProjectExpand::Url => "url".to_string(),
            ProjectExpand::ProjectKeys => "projectKeys".to_string(),
        }
    }
}

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// REST API link to project
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// ID of project
    #[serde(default)]
    pub id: String,
    
    /// Key of project
    #[serde(default)]
    pub key: String,

    /// Project description
    #[serde(default)]
    pub description: String,

    /// The project lead
    #[serde(default)]
    pub lead: Option<User>,

    /// Components inside the project
    #[serde(default)]
    pub components: Vec<Component>,

    /// Issue types connected to the project
    #[serde(rename = "issueTypes", default)]
    pub issue_types: Vec<IssueType>,

    /// Assignee type
    #[serde(default, rename = "assigneeType")]
    pub assignee_type: String,

    /// A list of versions inside the project
    #[serde(default)]
    pub versions: Vec<Version>,

    /// Name of the project
    #[serde(default)]
    pub name: String,

    /// A list of roles assigned to the project
    #[serde(default)]
    pub roles: BTreeMap<String, String>,

    /// A list of avatar urls associated with the project
    #[serde(rename = "avatarUrls", default)]
    pub avatar_urls: BTreeMap<String, String>,

    /// Project type
    #[serde(rename = "projectTypeKey", default)]
    pub project_type_key: String,

    /// Is the project archived
    #[serde(default)]
    pub archived: bool
}

impl Project {
    /// Fetches a project from key or id, for more information see:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/project-getProject
    pub fn from_key_or_id<K>(
        c: &Client,
        key: K,
        expand_opts: Option<Vec<ProjectExpand>>
    ) -> Response<Project>
    where
        K: Into<String>,
    {
        let url = format!("api/2/project/{}", key.into());
        let mut c = c.clone();
        let query = expand_to_hashmap(expand_opts.unwrap_or_default());

        c = c.add_query(query);
 
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Project {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// ============================================================================
// Private
// ============================================================================
fn expand_to_hashmap(e: Vec<ProjectExpand>) -> HashMap<String, String> {
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
    fn test_deserialize_results() {
        let results =
            fs::read_to_string("tests/assets/v2/project.json").expect("Unable to read in JSON file");
        let project: Project = serde_json::from_str(&results).unwrap();

        assert_eq!(project.self_link, "http://localhost:8080/rest/api/2/project/10000");
        assert_eq!(project.id, "10000");
        assert_eq!(project.key, "TEST");
        assert_eq!(project.description, "");
        
        match project.lead {
            Some(u) => assert_eq!(u.name, "xxx"),
            None => assert!(false)
        }

        assert_eq!(project.issue_types.len(), 5);
        assert_eq!(project.assignee_type, "UNASSIGNED");
        assert_eq!(project.versions.len(), 3);
        assert_eq!(project.name, "TEST");
        assert_eq!(project.roles.len(), 2);
    }
}
