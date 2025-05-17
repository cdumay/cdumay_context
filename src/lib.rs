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
//! - Type-safe error handling with the `cdumay_error::Error` struct
//!
//! # Example Usage
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde::{Serialize, Deserialize};
//! use cdumay_context::{Contextualize, Context};
//!
//! // Basic usage
//! let mut ctx = Context::new();
//! ctx.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
//!
//! ```
//!
//! # Error Handling
//!
//! The library provides a comprehensive error handling system through the `Error` enum:
//!
//! ```rust
//! use cdumay_context::{Context, ContextDump, Contextualize, UnExpectedError};
//! use cdumay_error::Error;
//! use rand::Rng;
//! use serde::{Serialize, Deserialize};
//! use std::collections::BTreeMap;
//!
//! fn example_error_handling() -> Result<(), Error> {
//!     let mut rng = rand::rng();
//!     let dice_roll: u8 = rng.random_range(1..=6);
//!     let mut ctx = Context::new();
//!     ctx.insert("env".to_string(), serde_value::Value::String("prod".to_string()));
//!
//!     // Generic error
//!     if dice_roll == 7 {
//!         return Err(UnExpectedError::new().set_message("Something went wrong".to_string()).set_details(ctx.dump()).into());
//!     }
//!
//!     Ok(())
//! }
//! ```

mod error;
pub use error::{GenericContextError, UnExpectedError};

mod context;
pub use context::{ContextDump, Context, Contextualize};
