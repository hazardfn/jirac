//! Handles JIRA credentials, currently only basic authentication is supported

// ============================================================================
// Use
// ============================================================================
use crate::Result;

// ============================================================================
// Public Enums
// ============================================================================
/// An enumeration of the authentication types available for the Jira subcommand
/// OAuth is currently not supported.
#[derive(Clone, Debug)]
pub enum Credentials {
    /// username and password credentials
    Basic(String, String),
}

impl Credentials {
    /// Builds new credentials given a username and password, if the password
    /// is not supplied it is requested through the tty automatically
    pub fn new<U, P>(username: U, password: P) -> Result<Credentials>
    where
        U: Into<String>,
        P: Into<String>,
    {
        Ok(Credentials::Basic(username.into(), password.into()))
    }
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_credentials() {
        let Credentials::Basic(u, p) =
            Credentials::new("test".to_string(), "password".to_string()).unwrap();

        assert_eq!(u, "test");
        assert_eq!(p, "password");
    }
}
