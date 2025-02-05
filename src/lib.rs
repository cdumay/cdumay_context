//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay-context on crates.io](https://img.shields.io/crates/v/cdumay-context)](https://crates.io/crates/cdumay-context)
//! [![cdumay-context on docs.rs](https://docs.rs/cdumay-context/badge.svg)](https://docs.rs/cdumay-context)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_context)
//!
//! `cdumay-context` is a lightweight and efficient Rust library designed for manipulating a context and exporting it into various formats. The library
//! provides simple methods to handle structured data and export it in widely used formats like `JSON`, `TOML`, and `YAML`.
//!
//! This makes it an ideal tool for developers working with configuration management, data serialization, or any use case requiring flexible
//! context manipulation.
//!
//! # Features
//!
//! * **Context Manipulation**: Store, modify, and query data within a context object.
//! * **Multiple Export Formats**: Export the context to JSON, TOML, or YAML formats.
//!
//! # Usage
//!
//! To utilize `cdumay-context` in your project, follow these steps:
//!
//! 1. **Add Dependencies**: To use `cdumay-context` in your project, add it to your Cargo.toml as a dependency:
//!
//! ```toml
//! [dependencies]
//! cdumay-context = "0.1"
//! ```
//!
//! 2. **Define Context**: The core feature of `cdumay-context` is the context. The context acts as a container where you can store key-value pairs of data.
//! Here’s how to create and manipulate it:
//!
//! ```rust
//! use cdumay_context::Context;
//! use serde_value::Value;
//!
//! fn main() {
//!     let mut context = Context::new();
//!     context.insert("name".to_string(), Value::String("John Doe".to_string()));
//!     context.insert("age".to_string(), Value::U8(30));
//!     dbg!(&context);
//!  }
//! ```
//!
//! 3. **Exporting the Context**: `cdumay-context` allows you to export the context into various formats like `JSON`, `TOML`, and `YAML`. You can use the
//! following methods to serialize the context:
//!
//! ```toml
//! [dependencies]
//! cdumay-context = {version = "0.1", features = ["json"] }
//! ```
//!
//! ```rust
//! use cdumay_context::Context;
//! use serde_value::Value;
//!
//! fn main() {
//!     let mut context = Context::new();
//!     context.insert("name".to_string(), Value::String("John Doe".to_string()));
//!     context.insert("age".to_string(), Value::U8(30));
//!     println!("{}", context.to_json(true).unwrap());
//!  }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "xml")]
mod xml;

/// Enum to represent various types of errors in the `cdumay_context` library.
#[derive(Debug)]
pub enum Error {
    /// A generic error that takes a string message.
    Generic(String),

    /// Error related to JSON processing, available if the "json" feature is enabled.
    #[cfg(feature = "json")]
    Json(String),

    /// Error related to TOML processing, available if the "toml" feature is enabled.
    #[cfg(feature = "toml")]
    Toml(String),

    /// Error related to XML processing, available if the "xml" feature is enabled.
    #[cfg(feature = "xml")]
    Xml(String),

    /// Error related to YAML processing, available if the "yaml" feature is enabled.
    #[cfg(feature = "yaml")]
    Yaml(String),
}


/// A type alias for `Result<T, Error>`.
///
/// This alias simplifies the usage of `Result` in the context of errors in your application.
/// Instead of writing out `std::result::Result<T, Error>` every time, you can now use `Result<T>`
/// for better readability and convenience.
///
/// # Example
/// ```
/// fn example() -> cdumay_context::Result<i32> {
///     Err(cdumay_context::Error::Xml("Invalid XML".to_string()))
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

/// A struct that represents a context, which stores key-value pairs in a BTreeMap.
/// The context can be serialized and deserialized using Serde.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    /// A BTreeMap that stores the inner key-value data.
    /// The `serde(flatten)` attribute means that this map will be serialized and deserialized
    /// as if its keys and values were directly on the `Context` struct, without nesting it.
    #[serde(flatten)]
    inner: BTreeMap<String, serde_value::Value>,
}

impl Context {
    /// Creates a new empty `Context` using the default implementation.
    /// This is equivalent to calling `Context::default()`.
    pub fn new() -> Context {
        Context::default()
    }

    /// Inserts a key-value pair into the `Context`.
    ///
    /// `k`: The key to insert (of type `String`).
    /// `v`: The value associated with the key, of type `serde_value::Value`.
    ///
    /// Example:
    /// ```
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
    /// assert_eq!(context.get("name").unwrap(), &serde_value::Value::String("Alice".to_string()));
    /// ```
    pub fn insert(&mut self, k: String, v: serde_value::Value) {
        self.inner.insert(k, v);
    }

    /// Retrieves a reference to the value associated with the given key.
    ///
    /// `k`: The key to look up (of type `&str`).
    ///
    /// Returns `Some(&serde_value::Value)` if the key exists, or `None` if it doesn't.
    ///
    /// Example:
    /// ```
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("age".to_string(), serde_value::Value::U64(30));
    /// if let Some(value) = context.get("age") {
    ///     assert_eq!(value, &serde_value::Value::U64(30));
    /// } else {
    ///     panic!("Key not found");
    /// }
    /// ```
    pub fn get(&self, k: &str) -> Option<&serde_value::Value> {
        self.inner.get(k)
    }

    /// Extends the `Context` by adding key-value pairs from another `BTreeMap`.
    ///
    /// `data`: A `BTreeMap<String, serde_value::Value>` containing key-value pairs to add.
    ///
    /// Example:
    /// ```
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("key1".to_string(), serde_value::Value::String("value1".to_string()));
    ///
    /// let mut extra_data = std::collections::BTreeMap::new();
    /// extra_data.insert("key2".to_string(), serde_value::Value::String("value2".to_string()));
    /// context.extend(extra_data);
    ///
    /// assert_eq!(context.get("key1").unwrap(), &serde_value::Value::String("value1".to_string()));
    /// assert_eq!(context.get("key2").unwrap(), &serde_value::Value::String("value2".to_string()));
    /// ```
    pub fn extend(&mut self, data: BTreeMap<String, serde_value::Value>) {
        self.inner.extend(data);
    }
}


impl Into<BTreeMap<String, serde_value::Value>> for Context {
    /// Converts a `Context` instance into a `BTreeMap<String, serde_value::Value>`.
    ///
    /// This implementation allows you to convert the `Context` directly into a `BTreeMap`
    /// when using `.into()`, for example in contexts where the map structure is needed.
    ///
    /// Example:
    /// ```
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("key1".to_string(), serde_value::Value::String("value1".to_string()));
    ///
    /// // Convert the Context into a BTreeMap
    /// let map: std::collections::BTreeMap<String, serde_value::Value> = context.into();
    ///
    /// // Verify the map contains the key-value pair
    /// assert_eq!(map.get("key1"), Some(&serde_value::Value::String("value1".to_string())));
    /// ```
    fn into(self) -> BTreeMap<String, serde_value::Value> {
        self.inner
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut ctx = Context::new();
        ctx.insert("foo".to_string(), serde_value::to_value("foo").unwrap());
        assert_eq!(
            ctx.get("foo").unwrap(),
            &serde_value::to_value("foo").unwrap()
        );
    }
    #[test]
    fn test_extend() {
        let mut ctx = Context::new();
        let mut data = BTreeMap::new();
        data.insert("bar".to_string(), serde_value::to_value("baz").unwrap());
        ctx.extend(data);
        assert_eq!(
            ctx.get("bar").unwrap(),
            &serde_value::to_value("baz").unwrap()
        );
    }
}
