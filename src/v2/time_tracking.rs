//! Some fields in Jira have a common strucure, we use this module to
//! deserialize them into their respective types

// ============================================================================
// Use
// ============================================================================
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTracking {
    /// Remaining time given the overall estimate in "x{m} y{h} z{s}" format
    /// for example: "2d 4h"
    #[serde(rename = "remainingEstimate", default)]
    pub remaining_estimate: String,

    /// Time spent on the issue in "x{m} y{h} z{s}" format
    /// for example: "2d 4h"
    #[serde(rename = "timeSpent", default)]
    pub time_spent: String,

    /// Remaining time given the overall estimate in seconds as integer
    /// for example: 24
    #[serde(rename = "remainingEstimateSeconds", default)]
    pub remaining_estimate_seconds: i64,

    /// Time spent on the issue in seconds as integer
    /// for example: 10
    #[serde(rename = "timeSpentSeconds", default)]
    pub time_spent_seconds: i64
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for TimeTracking {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}