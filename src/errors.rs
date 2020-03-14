//! Generic error handling module for the client

// ============================================================================
// Use
// ============================================================================
use reqwest::Error as HttpError;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::error::Error as SerdeError;
use std::collections::BTreeMap;
use std::io::Error as IoError;

// ============================================================================
// Public Structures
// ============================================================================
/// represents a general jira error response
#[derive(Deserialize, Debug)]
pub struct Errors {
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    pub errors: BTreeMap<String, String>,
}

// ============================================================================
// Public Enums
// ============================================================================
/// an enumeration over potential errors that may happen when sending a request
/// to jira
#[derive(Debug)]
pub enum Error {
    /// error associated with http request
    Http(HttpError),
    /// error associated IO
    IO(IoError),
    /// error associated with parsing or serializing
    Serde(SerdeError),
    /// client request errors
    Fault { code: StatusCode, errors: Errors },
    /// invalid credentials
    Unauthorized,
    /// Precondition failed
    PreconditionFailed,
    /// Forbidden action
    Forbidden,
    /// HTTP method is not allowed
    MethodNotAllowed,
    /// Page not found
    NotFound,
}

// ============================================================================
// Error Implementations
// ============================================================================
impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::Serde(error)
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IO(error)
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use crate::Error::*;

        match *self {
            Http(ref e) => writeln!(f, "Http Error: {}", e),
            IO(ref e) => writeln!(f, "IO Error: {}", e),
            Serde(ref e) => writeln!(f, "Serialization Error: {}", e),
            Fault {
                ref code,
                ref errors,
            } => writeln!(f, "Jira Client Error ({}):\n{:#?}", code, errors),
            _ => writeln!(f, "Could not connect to Jira: {}!", self),
        }
    }
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&dyn ::std::error::Error> {
        use crate::Error::*;

        match *self {
            Http(ref e) => Some(e),
            IO(ref e) => Some(e),
            Serde(ref e) => Some(e),
            Fault { .. } => None,
            _ => None,
        }
    }
}
