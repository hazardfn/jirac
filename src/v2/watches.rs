//! Represents watchers on a JIRA issue.

// ============================================================================
// Use
// ============================================================================
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Watches {
    /// REST API link to watchers
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Number of watchers
    #[serde(rename = "watchCount", default)]
    pub watch_count: i64,

    /// Are there watchers on this issue?
    #[serde(rename = "isWatching", default)]
    pub is_watching: bool
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Watches {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}