# cdumay-context

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay-context on crates.io](https://img.shields.io/crates/v/cdumay-context)](https://crates.io/crates/cdumay-context)
[![cdumay-context on docs.rs](https://docs.rs/cdumay-context/badge.svg)](https://docs.rs/cdumay-context)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_context)

`cdumay-context` is a lightweight and efficient Rust library designed for manipulating a context and exporting it into various formats. The library
provides simple methods to handle structured data and export it in widely used formats like `JSON`, `TOML`, and `YAML`.

This makes it an ideal tool for developers working with configuration management, data serialization, or any use case requiring flexible
context manipulation.

## Features

* **Context Manipulation**: Store, modify, and query data within a context object.
* **Multiple Export Formats**: Export the context to JSON, TOML, or YAML formats.

## Usage

To utilize `cdumay-context` in your project, follow these steps:

1. **Add Dependencies**: To use `cdumay-context` in your project, add it to your Cargo.toml as a dependency:

```toml
[dependencies]
cdumay-context = "0.1"
```

2. **Define Context**: The core feature of `cdumay-context` is the context. The context acts as a container where you can store key-value pairs of data.
HereÔÇÖs how to create and manipulate it:

```rust
use cdumay_context::Context;
use serde_value::Value;

fn main() {
    let mut context = Context::new();
    context.insert("name".to_string(), Value::String("John Doe".to_string()));
    context.insert("age".to_string(), Value::U8(30));
    dbg!(&context);
 }
```

3. **Exporting the Context**: `cdumay-context` allows you to export the context into various formats like `JSON`, `TOML`, and `YAML`. You can use the
following methods to serialize the context:

```toml
[dependencies]
cdumay-context = {version = "0.1", features = ["json"] }
```

```rust
use cdumay_context::Context;
use serde_value::Value;

fn main() {
    let mut context = Context::new();
    context.insert("name".to_string(), Value::String("John Doe".to_string()));
    context.insert("age".to_string(), Value::U8(30));
    println!("{}", context.to_json(true).unwrap());
 }
```
