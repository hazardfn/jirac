//! Represents an attachment in the JIRA system

// ============================================================================
// Use
// ============================================================================
use crate::v2::User;
use crate::Client;
use crate::Response;
use crate::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Public Structures
// ============================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// REST API link to attachment
    #[serde(rename = "self", default)]
    pub self_link: String,

    /// Filename of the attachment, e.g x.jpg
    #[serde(default)]
    pub filename: String,

    /// User that uploaded the attachment
    #[serde(default)]
    pub author: Option<User>,

    /// Date created in the following format: "2020-03-09T20:01:55.575+0000"
    #[serde(default)]
    pub created: String,

    /// Size of the attachment in bytes
    #[serde(default)]
    pub size: i64,

    /// MIME type of the attachment e.g. "image/jpeg"
    #[serde(rename = "mimeType", default)]
    pub mime_type: String,

    /// An arbitrary list of properties
    #[serde(default)]
    pub properties: BTreeMap<String, ::serde_json::Value>,

    /// Direct link to the content
    #[serde(default)]
    pub content: String,

    /// Direct link to content thumbnail
    #[serde(default)]
    pub thumbnail: String
}

impl Attachment {
    /// Fetches an attachment given its ID. For more info consult the api docs:
    /// https://docs.atlassian.com/software/jira/docs/api/REST/8.2.6/#api/2/attachment-getAttachment
    pub fn from_id<I>(
        c: &Client,
        id: I,
    ) -> Response<Attachment>
    where
        I: Into<String>,
    {
        let url = format!("api/2/attachment/{}", id.into());
        c.get(&url)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================
impl std::fmt::Display for Attachment {
    // This trait requires fmt with this signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize_results() {
        let results =
            fs::read_to_string("tests/assets/v2/attachment.json").expect("Unable to read in JSON file");
        let a: Attachment = serde_json::from_str(&results).unwrap();

        assert_eq!(a.self_link, "http://www.example.com/jira/rest/api/2.0/attachments/10000");
        assert_eq!(a.filename, "picture.jpg");
        assert_eq!(a.author.unwrap().name, "fred");
        assert_eq!(a.created, "2019-11-05T17:14:26.121+0000");
        assert_eq!(a.size, 23123);
        assert_eq!(a.mime_type, "image/jpeg");
        assert_eq!(a.content, "http://www.example.com/jira/attachments/10000");
        assert_eq!(a.thumbnail, "http://www.example.com/jira/secure/thumbnail/10000");
    }
}
