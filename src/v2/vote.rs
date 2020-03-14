//! A structure for votes on an issue, for retrieving votes tied to an issue
//! see v2::Issue

// ============================================================================
// Use
// ============================================================================
use crate::v2::User;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    /// Number of items in the list
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Number of votes for an issue
    #[serde(default)]
    pub votes: i64,

    /// Have people voted
    #[serde(default)]
    pub has_voted: bool,

    /// Generic list of items
    #[serde(default)]
    pub voters: Vec<User>
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Vote {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}