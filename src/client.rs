//! Client implementation for JIRA

// ============================================================================
// Use
// ============================================================================
use crate::credentials::Credentials;
use crate::Result;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

// ============================================================================
// Public Structures
// ============================================================================
/// Represents the Jira Client, it is passed around and facilities communication
/// with the REST API
#[derive(Clone, Debug)]
pub struct Client {
    pub host: String,
    pub client: reqwest::Client,
    pub credentials: Credentials,
}

impl Client {
    /// Creates a new instance of the JIRA client
    pub fn new<H>(host: H, credentials: Credentials) -> Client
    where
        H: Into<String>,
    {
        Client {
            host: host.into(),
            client: reqwest::Client::new(),
            credentials,
        }
    }

    pub fn put<S, D>(
        &self,
        api_name: &str,
        version: &str,
        endpoint: &str,
        query: Option<HashMap<String, String>>,
        headers: Option<HeaderMap>,
        body: S,
    ) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let data = serde_json::to_string::<S>(&body)?;

        self.request(
            Method::PUT,
            api_name,
            version,
            endpoint,
            query,
            headers,
            Some(data.into_bytes()),
        )
    }

    pub fn get<D>(
        &self,
        api_name: &str,
        version: &str,
        endpoint: &str,
        query: Option<HashMap<String, String>>,
        headers: Option<HeaderMap>,
    ) -> Result<D>
    where
        D: DeserializeOwned,
    {
        self.request(
            Method::GET,
            api_name,
            version,
            endpoint,
            query,
            headers,
            None,
        )
    }

    pub fn request<D>(
        &self,
        method: Method,
        api_name: &str,
        version: &str,
        endpoint: &str,
        query: Option<HashMap<String, String>>,
        headers: Option<HeaderMap>,
        body: Option<Vec<u8>>,
    ) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let query = if let Some(q) = query {
            unpack_query(q)
        } else {
            String::from("")
        };

        let headers = if let Some(mut h) = headers {
            h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .unwrap();
            h
        } else {
            HeaderMap::new()
        };

        let url = format!(
            "{}/rest/{}/{}/{}{}",
            self.host, api_name, version, endpoint, query
        );
        let req = self.client.request(method, &url);
        let builder = match self.credentials {
            Credentials::Basic(ref user, ref pass) => req
                .basic_auth(user.to_owned(), Some(pass.to_owned()))
                .headers(headers),
        };

        let mut res = match body {
            Some(body) => builder.body(body).send()?,
            _ => builder.send()?,
        };

        let body = res.text()?;
        let data = if body == "" { "null" } else { &body };

        Ok(serde_json::from_str::<D>(data)?)
    }
}

// ============================================================================
// Private
// ============================================================================
fn unpack_query(query: HashMap<String, String>) -> String {
    let mut ret = query.iter().fold(String::from("?"), |acc, (k, v)| {
        format!("{}{}={}&", acc, k, v)
    });

    ret.pop();
    ret
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_to_query() {
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert(String::from("q"), String::from("true"));
        query.insert(String::from("h"), String::from("other-text"));

        let unpacked = unpack_query(query);

        assert!(&unpacked.contains("q=true"));
        assert!(&unpacked.contains("h=other-text"));
    }

    #[test]
    fn test_empty_hashmap_empty_query() {
        let query: HashMap<String, String> = HashMap::new();
        assert_eq!(unpack_query(query), "");
    }
}
