//! Pagination follows a similar patter across the JIRA API, this structure
//! is used when pagination options are available.

// ============================================================================
// Use
// ============================================================================
use crate::Options;
use crate::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    /// REST API link to group
    #[serde(rename = "startAt", default)]
    pub start_at: i64,

    /// Name of the group
    #[serde(rename = "maxResults", default)]
    pub max_results: i64,

    /// URL to next page
    #[serde(rename = "nextPage", default)]
    pub next_link: String,

    /// Total groups
    #[serde(default)]
    pub total: i64,

    /// Is the last page
    #[serde(default)]
    pub is_last: bool,
}

impl Pagination {
    pub fn new(start: i64, max: i64) -> Pagination {
        Pagination {
            start_at: start,
            max_results: max,
            is_last: false,
            total: 0,
            next_link: String::from(""),
        }
    }

    pub fn next(&self) -> Option<Pagination> {
        if self.is_last {
            None
        } else {
            Some(Pagination {
                start_at: self.start_at + (self.max_results - 1),
                max_results: self.max_results,
                is_last: false,
                total: 0,
                next_link: String::from(""),
            })
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            start_at: 0,
            max_results: 50,
            is_last: false,
            total: 0,
            next_link: String::from(""),
        }
    }
}

impl Options for Pagination {
    fn to_query(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(String::from("startAt"), self.start_at.to_string());
        h.insert(String::from("maxResults"), self.max_results.to_string());
        h
    }
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_pagination() {
        let mut p = Pagination::new(2, 2);
        p = p.next().unwrap();

        assert_eq!(p.max_results, 2);
        assert_eq!(p.start_at, 3);
    }

    #[test]
    fn test_no_more_pages() {
        let mut p = Pagination::default();
        p.is_last = true;

        assert!(p.next().is_none());
    }
}
