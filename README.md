# cdumay_context

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_context on crates.io](https://img.shields.io/crates/v/cdumay_context)](https://crates.io/crates/cdumay_context)
[![cdumay_context on docs.rs](https://docs.rs/cdumay_context/badge.svg)](https://docs.rs/cdumay_context)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_context)

A flexible context management library that provides a trait-based approach for handling
key-value data with support for multiple serialization formats.

## Features

- Generic context management through the `Contextualize` trait and a `Context` struct
- Support for multiple serialization formats (with feature flags):
  - JSON (feature: "json")
  - TOML (feature: "toml")
  - YAML (feature: "yaml")
- Allow to dump context into `reqwest` headers using feature "http-headers"
- Type-safe error handling with the `Error` enum

## Example Usage

```rust
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use cdumay_context::{Contextualize, Error, Context};

// Basic usage
let mut ctx = Context::new();
ctx.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));

// JSON serialization (requires "json" feature)
#[cfg(feature = "json")]
{
    let json = ctx.to_json(true).unwrap();
    let ctx_from_json = Context::from_json(&json).unwrap();
    assert_eq!(ctx.get("name"), ctx_from_json.get("name"));
}

// TOML serialization (requires "toml" feature)
#[cfg(feature = "toml")]
{
    let toml = ctx.to_toml(true).unwrap();
    let ctx_from_toml = Context::from_toml(&toml).unwrap();
    assert_eq!(ctx.get("name"), ctx_from_toml.get("name"));
}
```

## Error Handling

The library provides a comprehensive error handling system through the `Error` enum:

```rust
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use cdumay_context::{Context, Contextualize, Error};
use rand::Rng;

fn example_error_handling() -> Result<(), Error> {
    let mut rng = rand::rng();
    let dice_roll: u8 = rng.random_range(1..=6);

    // Generic error
    if dice_roll == 7 {
        return Err(Error::Generic("Something went wrong".to_string()));
    }

    // JSON error (with "json" feature)
    #[cfg(feature = "json")]
    {
        let invalid_json = "{ invalid: json }";
        let result = Context::from_json(invalid_json);
        assert!(matches!(result, Err(Error::Json(_))));
    }
    Ok(())
}
```
