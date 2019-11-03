//! Generic module for options, options are optional values that can be sent to
//! the API

// ============================================================================
// Use
// ============================================================================
use std::collections::HashMap;

// ============================================================================
// Traits
// ============================================================================
pub trait Options {
    fn to_query(&self) -> HashMap<String, String>;
}
