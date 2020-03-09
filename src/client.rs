//! Client implementation for JIRA

// ============================================================================
// Use
// ============================================================================
use crate::Credentials;
use crate::Options;
use crate::Resp;
use crate::Response;
use crate::Serialize;
use crate::{Error, Errors};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
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
    pub headers: HeaderMap,
    pub query: HashMap<String, String>,
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
            headers: HeaderMap::new(),
            query: HashMap::new(),
        }
    }

    /// Add request headers before sending your request
    pub fn add_headers(&mut self, headers: HashMap<&'static str, String>) {
        let mut h = self.headers.clone();

        h = headers.iter().fold(h, |mut acc, (k, v)| {
            acc.insert(*k, HeaderValue::from_str(v).unwrap());
            acc
        });

        self.headers = h;
    }

    /// Add a single header given a string value
    pub fn add_header(&mut self, header: &'static str, value: String) {
        self.headers
            .insert(header, HeaderValue::from_str(&value).unwrap());
    }

    /// Add query string arguments before sending your request
    pub fn add_query(mut self, query: HashMap<String, String>) -> Self {
        let mut q = self.query;
        q.extend(query);
        self.query = q;

        self
    }

    /// Unpacks options into a HashMap<String, String> allowing them to be easily
    /// represented in a query string.
    pub fn unpack_options(opts: Vec<&dyn Options>) -> HashMap<String, String> {
        opts.iter().fold(HashMap::new(), |acc, o| {
            let mut h = acc;
            h.extend(o.to_query());
            h
        })
    }

    /// Unpacks a HashMap<String, String> into a query string.
    pub fn unpack_query(query: &HashMap<String, String>) -> String {
        let mut ret = query.iter().fold(String::from("?"), |acc, (k, v)| {
            format!("{}{}={}&", acc, k, v)
        });
        ret.pop();
        ret
    }

    pub fn put<S, D>(&self, url: &str, body: S) -> Response<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let data = serde_json::to_string::<S>(&body)?;

        self.request::<D>(Method::PUT, url, Some(data.into_bytes()))
    }

    pub fn post<S, D>(&self, url: &str, body: S) -> Response<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let data = serde_json::to_string::<S>(&body)?;

        self.request::<D>(Method::POST, url, Some(data.into_bytes()))
    }

    pub fn get<D>(&self, url: &str) -> Response<D>
    where
        D: DeserializeOwned,
    {
        self.request::<D>(Method::GET, url, None)
    }

    pub fn request<D>(&self, method: Method, url: &str, body: Option<Vec<u8>>) -> Response<D>
    where
        D: DeserializeOwned,
    {
        let query = Client::unpack_query(&self.query);
        let url = format!("{}/rest/{}{}", self.host, url, query);
        let req = self.client.request(method, &url);
        let builder = match self.credentials {
            Credentials::Basic(ref user, ref pass) => req
                .basic_auth(user.to_owned(), Some(pass.to_owned())),
            Credentials::OAuth(ref token) => req
                .bearer_auth(token)
        }.header(CONTENT_TYPE, HeaderValue::from_static("application/json")).headers(self.headers.clone());

        let mut res = match body {
            Some(body) => builder.body(body).send()?,
            _ => builder.send()?,
        };

        let body = res.text()?;
        let data = if body == "" { "null" } else { &body };

        match res.status() {
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::METHOD_NOT_ALLOWED => Err(Error::MethodNotAllowed),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::PRECONDITION_FAILED => Err(Error::PreconditionFailed),
            StatusCode::FORBIDDEN => Err(Error::Forbidden),
            client_err if client_err.is_client_error() => Err(Error::Fault {
                code: res.status(),
                errors: serde_json::from_str::<Errors>(&body)?,
            }),
            _ => Ok(Resp {
                data: serde_json::from_str::<D>(data)?,
                headers: res.headers().clone(),
            }),
        }
    }
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

        let unpacked = Client::unpack_query(&query);

        assert!(&unpacked.contains("q=true"));
        assert!(&unpacked.contains("h=other-text"));
    }

    #[test]
    fn test_empty_hashmap_empty_query() {
        let query: HashMap<String, String> = HashMap::new();
        assert_eq!(Client::unpack_query(&query), "");
    }
}
