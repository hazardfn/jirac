//! Represents an issue in Jira, as the fields here can be incredibly dynamic
//! it works a little differently to the other APIs in that you have to
//! consciously extract fields from the struct as opposed to them being readily
//! available.
//!
//! This allows you to destructure custom fields for example in your code.

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Attachment, Component, Changelog, IssueType, IssueLink};
use crate::v2::{PaginatedComments, PaginatedWorklog, Pagination, Priority};
use crate::v2::{Progress, Project, Resolution, Status, TimeTracking, User};
use crate::v2::{Version, Vote, Watches};
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};
use crate::QueryOptions;
use std::collections::BTreeMap;
use std::collections::HashMap;

// ============================================================================
// Public Enums
// ============================================================================
pub enum IssueExpand {
    RenderedFields,
    Names,
    Schema,
    Transitions,
    Operations,
    Editmeta,
    Changelog,
    VersionedRepresentations,
}

impl QueryOptions for IssueExpand {
    fn to_string(&self) -> String {
        match &self {
            IssueExpand::RenderedFields => "renderedFields".to_string(),
            IssueExpand::Names => "names".to_string(),
            IssueExpand::Schema => "schema".to_string(),
            IssueExpand::Transitions => "transitions".to_string(),
            IssueExpand::Operations => "operations".to_string(),
            IssueExpand::Editmeta => "editmeta".to_string(),
            IssueExpand::Changelog => "changelog".to_string(),
            IssueExpand::VersionedRepresentations => "versionedRepresentations".to_string(),
        }
    }
}

// ============================================================================
// Public Structures
// ============================================================================
/// You should instantiate this struct with a list of strings which represent
/// field names, to signify you want to include the field just supply the field
/// name - to exclude the field place a '-' in front of the field name.alloc
/// 
/// Examples:
/// *all - include all fields
/// *navigable - include just navigable fields
/// summary,comment - include just the summary and comments
/// -comment - include everything except comments (the default is *all for get-issue)
/// *all,-comment - include everything except comments
pub struct IssueFieldOptions(Vec<String>);

impl Default for IssueFieldOptions {
    fn default() -> Self {
        IssueFieldOptions(vec!["*all".to_string()])
    }
}

impl QueryOptions for IssueFieldOptions {
    fn to_string(&self) -> String {
        let IssueFieldOptions(l) = &self;
        let mut ret = l
            .iter()
            .fold(String::new(), |acc, l| format!("{}{},", acc, l));

        ret.pop();
        ret
    }
}

/// You should instantiate this struct with a list of strings which represent
/// property names, as with fields to signify you want to include the property
/// just supply the property name - to exclude the field place a '-' in front
/// of the property name
pub struct IssuePropertyOptions(Vec<String>);

impl Default for IssuePropertyOptions {
    fn default() -> Self {
        IssuePropertyOptions(vec![])
    }
}

impl QueryOptions for IssuePropertyOptions {
    fn to_string(&self) -> String {
        let IssuePropertyOptions(l) = &self;
        let mut ret = l
            .iter()
            .fold(String::new(), |acc, l| format!("{}{},", acc, l));

        ret.pop();
        ret
    }
}

/// TODO: Find out what this is for, the API documentation mentions it only by
/// name and that it defaults to false. Google was not helpful.
pub struct IssueUpdateHistory(bool);

impl Default for IssueUpdateHistory {
    fn default() -> Self {
        IssueUpdateHistory(false)
    }
}

impl QueryOptions for IssueUpdateHistory {
    fn to_string(&self) -> String {
        let IssueUpdateHistory(b) = &self;
        b.to_string()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IssueFields {
    /// The issue type
    #[serde(default, rename = "issuetype")]
    pub issue_type: Option<IssueType>,

    /// Components belonging to the issue
    #[serde(default)]
    pub components: Vec<Component>,

    /// Time logged on the issue
    #[serde(rename = "timespent", default)]
    pub time_spent: Option<i64>,

    /// Original estimate logged on the issue
    #[serde(rename = "timeoriginalestimate", default)]
    pub time_original_estimate: Option<i64>,

    /// Description of the issue
    #[serde(default)]
    pub description: String,

    /// Project issue is part of
    #[serde(default)]
    pub project: Option<Project>,
    
    /// Fix versions assigned to the issue
    #[serde(rename = "fixVersions", default)]
    pub fix_versions: Vec<Version>,

    /// Aggregate time spent on the issue
    #[serde(rename = "aggregatetimespent", default)]
    pub aggregate_time_spent: Option<i64>,

    /// Resolution
    #[serde(default)]
    pub resolution: Option<Resolution>,

    /// Time tracking information (time spent, overall estimate etc.)
    #[serde(default)]
    pub timetracking: Option<TimeTracking>,

    /// A list of attachments in the issue.
    #[serde(default)]
    pub attachment: Vec<Attachment>,

    /// Aggregated time estimate in seconds
    #[serde(rename = "aggregatetimeestimate", default)]
    pub aggregate_time_estimate: Option<i64>,

    /// Date the issue was resolved (put into a resolution status)
    /// in the format: "2020-03-09T20:40:15.922+0000"
    #[serde(rename = "resolutiondate", default)]
    pub resolution_date: Option<String>,

    /// Work ratio
    #[serde(rename = "workratio", default)]
    pub work_ratio: Option<i64>,

    /// Summary of the issue
    #[serde(default)]
    pub summary: String,

    /// Date the issue was last viewed in the format:
    /// "2020-03-09T20:40:15.922+0000"
    #[serde(rename = "lastViewed", default)]
    pub last_viewed: Option<String>,

    /// Watcher details (how many people are watching this issue etc.)
    #[serde(default)]
    pub watches: Option<Watches>,

    /// Creator of the issue
    #[serde(default)]
    pub creator: Option<User>,

    /// Subtasks underneath the issue
    #[serde(default)]
    pub subtasks: Vec<Issue>,

    /// Date the issue was created in format: "2020-03-08T14:49:58.599+0000"
    #[serde(default)]
    pub created: String,

    /// User who reported the issue
    #[serde(default)]
    pub reporter: Option<User>,

    /// Aggregated progress based on time estimation in the ticket
    #[serde(rename = "aggregateprogress", default)]
    pub aggregate_progress: Option<Progress>,

    /// Priority assigned to the issue
    #[serde(default)]
    pub priority: Option<Priority>,

    /// A list of labels assigned to the issue
    #[serde(default)]
    pub labels: Vec<String>,

    /// Environment description
    #[serde(default)]
    pub environment: Option<String>,

    /// Time estimate in seconds
    #[serde(rename = "timeestimate", default)]
    pub time_estimate: Option<i64>,

    /// Original estimated time in seconds
    #[serde(rename = "aggregatetimeoriginalestimate", default)]
    pub aggregate_time_original_estimate: Option<i64>,

    /// A list of versions attributed to the issue
    #[serde(default)]
    pub versions: Vec<Version>,

    /// Due date of the issue
    #[serde(rename = "duedate", default)]
    pub due_date: Option<String>,

    /// Progress on an issue
    #[serde(default)]
    pub progress: Option<Progress>,

    /// Paginated comments
    #[serde(default)]
    pub comment: Option<PaginatedComments>,

    /// Links to other issues.
    #[serde(rename = "issuelinks", default)]
    pub issue_links: Vec<IssueLink>,

    /// Votes on the issue.
    #[serde(default)]
    pub votes: Option<Vote>,    

    /// A list of worklogs tied to the issue.
    #[serde(default)]
    pub worklog: Option<PaginatedWorklog>,

    /// Person assigned to the issue.
    #[serde(default)]
    pub assignee: Option<User>,

    /// Date time the issue was last updated in the format: 
    /// "2020-03-10T16:27:20.772+0000"
    #[serde(default)]
    pub updated: String,

    /// Status of the issue
    #[serde(default)]
    pub status: Option<Status>,

    /// Flatten
    #[serde(default, flatten)]
    pub others: BTreeMap<String, ::serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    /// A link to the issue
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// The issue key in the format XXX-123
    #[serde(default)]
    pub key: String,

    /// The internal id of the ticket
    #[serde(default)]
    pub id: String,

    /// Fields inside the issue supported by this library, if you are looking
    /// for custom data or otherwise can't find what you need check the others
    /// key
    pub fields: IssueFields,

    /// Anything not covered will be flattened for access here
    #[serde(flatten, default)]
    pub others: BTreeMap<String, ::serde_json::Value>,

    /// A chronical of the changes made to the issue.
    #[serde(default)]
    pub changelog: Option<Changelog>,
}

impl Issue {
    /// Fetches a paginated list of issues given an issue key (XXX-123). For more
    /// info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/issue-getIssue
    pub fn from_key<I>(
        c: &Client,
        key: I,
        expand_opts: Option<Vec<IssueExpand>>,
        field_opts: Option<IssueFieldOptions>,
        property_opts: Option<IssuePropertyOptions>,
        update_history: Option<IssueUpdateHistory>,
        page: Option<Pagination>,
    ) -> Response<Issue>
    where
        I: Into<String>,
    {
        let mut c = c.clone();
        let mut query = Client::unpack_options(vec![&page.unwrap_or_default()]);

        query.insert("fields".to_string(), field_opts.unwrap_or_default().to_string());
        query.insert("properties".to_string(), property_opts.unwrap_or_default().to_string());
        query.insert("updateHistory".to_string(), update_history.unwrap_or_default().to_string());
        query.extend(expand_to_hashmap(expand_opts.unwrap_or_default()));
        c = c.add_query(query);

        let endpoint = format!("api/2/issue/{}", key.into());

        c.get(&endpoint)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Issue {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for IssueFields {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// ============================================================================
// Private
// ============================================================================
fn expand_to_hashmap(e: Vec<IssueExpand>) -> HashMap<String, String> {
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
            fs::read_to_string("tests/assets/v2/issue.json").expect("Unable to read in JSON file");
        let _issue: Issue = serde_json::from_str(&results).unwrap();
    }
}
