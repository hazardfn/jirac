//! Some fields in Jira have a common strucure, we use this module to
//! deserialize them into their respective types

// ============================================================================
// Use
// ============================================================================
use serde::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// Number of items in the list
    #[serde(default)]
    pub size: usize,

    /// Generic list of items
    #[serde(default)]
    pub items: ::serde_json::Value,
}
