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
    /// Supply the access token here, your app should be responsible for
    /// defining scope and doing the OAuth dance.
    OAuth(String)
}

impl Credentials {
    /// Builds new credentials given a username and password, if the password
    /// is not supplied it is requested through the tty automatically
    pub fn new_basic<U, P>(username: U, password: P) -> Result<Credentials>
    where
        U: Into<String>,
        P: Into<String>,
    {
        Ok(Credentials::Basic(username.into(), password.into()))
    }

    /// Builds new credentials given an OAuth access token, you must do the
    /// OAuth dance as part of your application and pass the access token in.
    /// 
    /// The reason for this is that I do not know the scope of your application
    /// and it would be improper for this library to add infinite scoping to
    /// cover all its potential use-cases.
    /// 
    /// Whatever you supply here will be placed in the Bearer Auth header
    pub fn new_oauth<A>(access_token: A) -> Result<Credentials>
    where
        A: Into<String>
    {
        Ok(Credentials::OAuth(access_token.into()))
    }
}

// ============================================================================
// Tests
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_basic_credentials() {
        if let Credentials::Basic(u, p) =
            Credentials::new_basic("test".to_string(), "password".to_string()).unwrap() {

                assert_eq!(u, "test");
                assert_eq!(p, "password");
            }
    }

    #[test]
    fn test_instantiate_oauth_credentials() {
        if let Credentials::OAuth(a) = 
            Credentials::new_oauth("some_token".to_string()).unwrap() {
                assert_eq!(a, "some_token");
            }
    }
}
