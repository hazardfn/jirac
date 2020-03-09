//! A representation of JIRA's history format

// ============================================================================
// Use
// ============================================================================
use crate::v2::User;
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct History {
    /// The author of a change
    pub author: User,

    /// When the change was made
    #[serde(default)]
    pub created: String,

    /// A list of items changed including previous and new values
    #[serde(default)]
    pub items: Vec<HistoryItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryItem {
    /// Name of field changed
    #[serde(default)]
    pub field: String,

    /// What the field was changed from in its object form.
    /// Depending on what field this was you may be able to deserialize it
    /// further but that would have to be done inside your application where
    /// you are able to determine the context.
    #[serde(default)]
    pub from: ::serde_json::Value,

    /// What the field was changed from in string form.
    #[serde(rename = "fromString", default)]
    pub from_string: String,

    /// What the field was changed to in its object form.
    /// Depending on what field this was you may be able to deserialize it
    /// futher but that would have to be done inside your application where you
    /// are able to determine the context.
    #[serde(default)]
    pub to: ::serde_json::Value,

    /// What the field was changed to in string form.
    #[serde(rename = "toString", default)]
    pub to_string: String,
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for History {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for HistoryItem {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
