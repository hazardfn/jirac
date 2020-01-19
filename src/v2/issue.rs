//! Represents an issue in Jira, as the fields here can be incredibly dynamic
//! it works a little differently to the other APIs in that you have to
//! consciously extract fields from the struct as opposed to them being readily
//! available.
//!
//! This allows you to destructure custom fields for example in your code.

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Pagination, User};
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
pub struct Changelog {
    /// A list of changes
    #[serde(default)]
    pub histories: Vec<History>,
}

impl Default for Changelog {
    fn default() -> Self {
        Changelog { histories: vec![] }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct History {
    /// The author of a change
    pub author: User,

    /// When the change was made
    #[serde(default)]
    pub created: String,

    /// A list of items changed including previous and new values
    #[serde(default)]
    pub items: Vec<HistoryItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryItem {
    /// Name of field changed
    #[serde(default)]
    pub field: String,

    /// What the field was changed from in its object form.
    /// Depending on what field this was you may be able to deserialize it
    /// further but that would have to be done inside your application where
    /// you are able to determine the context.
    #[serde(default)]
    pub from: ::serde_json::Value,

    /// What the field was changed from in string form.
    #[serde(rename = "fromString", default)]
    pub from_string: String,

    /// What the field was changed to in its object form.
    /// Depending on what field this was you may be able to deserialize it
    /// futher but that would have to be done inside your application where you
    /// are able to determine the context.
    #[serde(default)]
    pub to: ::serde_json::Value,

    /// What the field was changed to in string form.
    #[serde(rename = "toString", default)]
    pub to_string: String,
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

    /// Fields inside the issue, contains a lot of information including
    /// custom fields
    #[serde(default)]
    pub fields: BTreeMap<String, ::serde_json::Value>,

    /// A chronical of the changes made to the issue.
    #[serde(default)]
    pub changelog: Changelog,
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
        let issue: Issue = serde_json::from_str(&results).unwrap();

        println!("{}", issue);
    }
}
