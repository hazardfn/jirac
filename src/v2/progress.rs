//! A structure representing the progress on a ticket based on time
//! estimations given.

// ============================================================================
// Use
// ============================================================================
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    /// Progress in seconds
    #[serde(default)]
    pub progress: i64,

    /// Total estimated time in seconds
    #[serde(default)]
    pub total: i64,

    /// Percentage of progress (progress / total)
    #[serde(default)]
    pub percent: i64
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Progress {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}