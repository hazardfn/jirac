//! A representation of JIRA's changelog format

// ============================================================================
// Use
// ============================================================================
use crate::v2::History;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct Changelog {
    /// A list of changes
    #[serde(default)]
    pub histories: Vec<History>
}

impl Default for Changelog {
    fn default() -> Self {
        Changelog { histories: vec![] }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Changelog {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}