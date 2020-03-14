//! Represents a worklog entry in the JIRA system

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Pagination, User};
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Worklog {
    /// REST API link to the worklog entry
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Author of the worklog entry
    #[serde(default)]
    pub author: Option<User>,

    /// Author of the last update to the worklog entry
    #[serde(rename = "updateAuthor", default)]
    pub update_author: Option<User>,

    /// Comment given for the worklog
    #[serde(default)]
    pub comment: String,

    /// Date the worklog was created in format: "2020-03-08T16:40:18.010+0000"
    #[serde(default)]
    pub created: String,

    /// Date the worklog was updated in format: "2020-03-08T16:40:18.010+0000"
    #[serde(default)]
    pub updated: String,

    /// Date the worklog was started in format: "2020-03-08T16:40:18.010+0000"
    #[serde(default)]
    pub started: Option<String>,

    /// Time spent on the issue associated with this worklog in format: "2d 4h"
    #[serde(rename = "timeSpent", default)]
    pub time_spent: Option<String>,

    /// Time spent in seconds on the issue associated with this worklog
    #[serde(rename = "timeSpentSeconds", default)]
    pub time_spent_seconds: Option<i64>,

    /// ID of this worklog
    #[serde(default)]
    pub id: String,

    /// ID of the associated issue
    #[serde(rename = "issueId", default)]
    pub issue_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedWorklog {
    /// A list of worklogs
    #[serde(default)]
    pub worklogs: Vec<Worklog>,

    /// Pagination fields
    #[serde(flatten, default)]
    pub pagination: Pagination
}

impl Worklog {
    /// Fetches worklogs given a list of worklog ids. For more info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/worklog-getWorklogsForIds
    pub fn from_ids<I>(
        c: &Client,
        ids: I,
    ) -> Response<Vec<Worklog>>
    where
        I: Into<Vec<i64>>,
    {
        #[derive(Serialize, Deserialize)]
        struct Request {
            ids: Vec<i64>
        }

        c.post("api/2/worklog/list", Request {ids: ids.into()})
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Worklog {
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
            fs::read_to_string("tests/assets/v2/worklog.json").expect("Unable to read in JSON file");
        let w: Worklog = serde_json::from_str(&results).unwrap();

        assert_eq!(w.self_link, "http://localhost:8080/rest/api/2/issue/10000/worklog/10000");
        assert_eq!(w.author.unwrap().name, "xxx");
        assert_eq!(w.update_author.unwrap().name, "xxx");
        assert_eq!(w.comment, "");
        assert_eq!(w.created, "2020-03-08T16:40:18.010+0000");
        assert_eq!(w.updated, "2020-03-08T16:40:18.010+0000");
        assert_eq!(w.started.unwrap(), "2020-03-08T16:40:00.000+0000");
        assert_eq!(w.time_spent.unwrap(), "2d 4h");
        assert_eq!(w.time_spent_seconds.unwrap(), 72000);
        assert_eq!(w.id, "10000");
        assert_eq!(w.issue_id, "10000");

    }
}