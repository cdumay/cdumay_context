//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_context on crates.io](https://img.shields.io/crates/v/cdumay_context)](https://crates.io/crates/cdumay_context)
//! [![cdumay_context on docs.rs](https://docs.rs/cdumay_context/badge.svg)](https://docs.rs/cdumay_context)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_context)
//!
//! A flexible context management library that provides a trait-based approach for handling
//! key-value data with support for multiple serialization formats.
//!
//! # Features
//!
//! - Generic context management through the `Contextualize` trait and a `Context` struct
//! - Support for multiple serialization formats (with feature flags):
//!   - JSON (feature: "json")
//!   - TOML (feature: "toml")
//!   - YAML (feature: "yaml")
//! - Allow to dump context into `reqwest` headers using feature "http-headers"
//! - Type-safe error handling with the `Error` enum
//!
//! # Example Usage
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde::{Serialize, Deserialize};
//! use cdumay_context::{Contextualize, Error, Context};
//!
//! // Basic usage
//! let mut ctx = Context::new();
//! ctx.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
//!
//! // JSON serialization (requires "json" feature)
//! #[cfg(feature = "json")]
//! {
//!     let json = ctx.to_json(true).unwrap();
//!     let ctx_from_json = Context::from_json(&json).unwrap();
//!     assert_eq!(ctx.get("name"), ctx_from_json.get("name"));
//! }
//!
//! // TOML serialization (requires "toml" feature)
//! #[cfg(feature = "toml")]
//! {
//!     let toml = ctx.to_toml(true).unwrap();
//!     let ctx_from_toml = Context::from_toml(&toml).unwrap();
//!     assert_eq!(ctx.get("name"), ctx_from_toml.get("name"));
//! }
//! ```
//!
//! # Error Handling
//!
//! The library provides a comprehensive error handling system through the `Error` enum:
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde::{Serialize, Deserialize};
//! use cdumay_context::{Context, Contextualize, Error};
//! use rand::Rng;
//!
//! fn example_error_handling() -> Result<(), Error> {
//!     let mut rng = rand::rng();
//!     let dice_roll: u8 = rng.random_range(1..=6);
//!
//!     // Generic error
//!     if dice_roll == 7 {
//!         return Err(Error::Generic("Something went wrong".to_string()));
//!     }
//!
//!     // JSON error (with "json" feature)
//!     #[cfg(feature = "json")]
//!     {
//!         let invalid_json = "{ invalid: json }";
//!         let result = Context::from_json(invalid_json);
//!         assert!(matches!(result, Err(Error::Json(_))));
//!     }
//!     Ok(())
//! }
//! ```

mod error;
pub use error::Error;

mod context;
pub use context::{Context, Contextualize};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    
    #[test]
    fn test_basic_operations() {
        let mut ctx = Context::new();
        
        // Test insert and get
        ctx.insert("key1".to_string(), serde_value::Value::String("value1".to_string()));
        assert_eq!(
            ctx.get("key1").unwrap(),
            &serde_value::Value::String("value1".to_string())
        );

        // Test extend
        let mut data = BTreeMap::new();
        data.insert("key2".to_string(), serde_value::Value::U64(42));
        ctx.extend(data);

        assert_eq!(ctx.get("key2").unwrap(), &serde_value::Value::U64(42));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        let mut ctx = Context::new();
        ctx.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
        ctx.insert("age".to_string(), serde_value::Value::U64(30));

        // Test JSON serialization/deserialization
        let json = ctx.to_json(true).unwrap();
        let ctx_from_json = Context::from_json(&json).unwrap();
        
        assert_eq!(ctx.get("name"), ctx_from_json.get("name"));
        assert_eq!(ctx.get("age"), ctx_from_json.get("age"));
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_serialization() {
        let mut ctx = Context::new();
        ctx.insert("name".to_string(), serde_value::Value::String("Bob".to_string()));
        ctx.insert("age".to_string(), serde_value::Value::I64(25));

        // Test TOML serialization/deserialization
        let toml = ctx.to_toml(true).unwrap();
        let ctx_from_toml = Context::from_toml(&toml).unwrap();
        
        assert_eq!(ctx.get("name"), ctx_from_toml.get("name"));
        assert_eq!(ctx.get("age"), ctx_from_toml.get("age"));
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_serialization() {
        let mut ctx = Context::new();
        ctx.insert("name".to_string(), serde_value::Value::String("Charlie".to_string()));
        ctx.insert("age".to_string(), serde_value::Value::U64(35));

        // Test YAML serialization/deserialization
        let yaml = ctx.to_yaml().unwrap();
        let ctx_from_yaml = Context::from_yaml(&yaml).unwrap();
        
        assert_eq!(ctx.get("name"), ctx_from_yaml.get("name"));
        assert_eq!(ctx.get("age"), ctx_from_yaml.get("age"));
    }
}
