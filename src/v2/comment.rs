//! A structure representing a comment on a ticket

// ============================================================================
// Use
// ============================================================================
use crate::v2::{Pagination, User};
use crate::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    /// REST API link to comment
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// ID of comment
    #[serde(default)]
    pub id: String,

    /// Author of comment
    #[serde(default)]
    pub author: Option<User>,

    /// Body of comment
    #[serde(default)]
    pub body: String,

    /// Author of last update to the comment
    #[serde(rename = "updateAuthor", default)]
    pub update_author: Option<User>,

    /// Date created in format "2020-03-10T16:14:17.856+0000",
    #[serde(default)]
    pub created: String,

    /// Date last updated in format "2020-03-10T16:14:17.856+0000",
    #[serde(default)]
    pub updated: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedComments {
    /// A list of comments
    #[serde(default)]
    pub comments: Vec<Comment>,

    /// Pagination
    #[serde(flatten, default)]
    pub pagination: Option<Pagination>
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Comment {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for PaginatedComments {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}