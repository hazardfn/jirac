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
pub struct Item {
    /// Number of items in the list
    #[serde(default)]
    pub size: i64,

    /// Generic list of items
    #[serde(default)]
    pub items: ::serde_json::Value,
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Item {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}