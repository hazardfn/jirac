//! Pagination follows a similar patter across the JIRA API, this structure
//! is used when pagination options are available.

// ============================================================================
// Use
// ============================================================================
use serde::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    /// REST API link to group
    #[serde(rename = "startAt", default)]
    pub start_at: usize,

    /// Name of the group
    #[serde(rename = "maxResults", default)]
    pub max_results: usize,

    /// URL to next page
    #[serde(rename = "nextPage", default)]
    pub next_link: String,

    /// Total groups
    #[serde(default)]
    pub total: usize,

    /// Is the last page
    #[serde(default)]
    pub is_last: bool,
}

impl Pagination {
    pub fn new(start: usize, max: usize) -> Pagination {
        Pagination {
            start_at: start,
            max_results: max,
            is_last: false,
            total: 0,
            next_link: String::from(""),
        }
    }
}
