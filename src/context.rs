//! Context management trait and implementations.
//!
//! This module provides the [`Contextualize`] trait, which defines a generic interface for
//! managing key-value data with support for various serialization formats.
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use cdumay_error::{Error, ErrorConverter};

/// A trait for managing key-value context data with serialization support.
///
/// The `Contextualize` trait provides a standardized interface for managing key-value data
/// with support for various serialization formats (JSON, TOML, YAML) through
/// feature flags.
///
/// # Type Parameters
///
/// The implementing type must be:
/// - `Sized`: Have a known size at compile time
/// - `Serialize`: Implement serde's Serialize trait
///
/// # Examples
///
/// Basic implementation:
/// ```rust
/// use std::collections::BTreeMap;
/// use serde::{Deserialize, Serialize};
/// use cdumay_context::Contextualize;
///
/// #[derive(Default, Serialize, Deserialize)]
/// struct MyContext {
///     data: BTreeMap<String, serde_value::Value>
/// }
///
/// impl Contextualize for MyContext {
///     fn new() -> Self {
///         Self::default()
///     }
///
///     fn insert(&mut self, k: String, v: serde_value::Value) {
///         self.data.insert(k, v);
///     }
///
///     fn get(&self, k: &str) -> Option<&serde_value::Value> {
///         self.data.get(k)
///     }
///
///     fn extend(&mut self, data: BTreeMap<String, serde_value::Value>) {
///         self.data.extend(data);
///     }
///
///     fn inner(&self) -> BTreeMap<String, serde_value::Value> {
///         self.data.clone()
///     }
/// }
/// ```
pub trait Contextualize: Sized + Serialize {
    /// Creates a new empty context.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the implementing type.
    fn new() -> Self;

    /// Inserts a key-value pair into the context.
    ///
    /// If the context already had this key present, the value is updated.
    ///
    /// # Parameters
    ///
    /// * `k` - The key to insert
    /// * `v` - The value to associate with the key
    fn insert(&mut self, k: String, v: serde_value::Value);

    /// Retrieves a reference to the value corresponding to the key.
    ///
    /// # Parameters
    ///
    /// * `k` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns `Some(&Value)` if the key exists, `None` otherwise.
    fn get(&self, k: &str) -> Option<&serde_value::Value>;

    /// Extends the context with the contents of another map.
    ///
    /// # Parameters
    ///
    /// * `data` - A map of key-value pairs to add to the context
    fn extend(&mut self, data: BTreeMap<String, serde_value::Value>);

    /// Returns a clone of the internal key-value store.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeMap` containing all key-value pairs in the context.
    fn inner(&self) -> BTreeMap<String, serde_value::Value>;

    /// Creates a new context from a JSON string.
    ///
    /// This method is only available when the "json" feature is enabled.
    ///
    /// # Parameters
    ///
    /// * `json` - A string containing valid JSON data
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, Error>` which is:
    /// * `Ok(context)` containing the parsed context on success
    /// * `Err(e)` containing the error on failure
    ///
    /// # Example
    ///
    /// ```rust
    /// use cdumay_context::Contextualize;
    /// use cdumay_error::Error;
    ///
    /// #[cfg(feature = "json")]
    /// fn example<T: Contextualize>(json: &str) -> Result<T, Error> {
    ///     let ctx = T::from_json(json)?;
    ///     Ok(ctx)
    /// }
    /// ```
    #[cfg(feature = "json")]
    fn from_json(json: &str) -> Result<Self, Error> {
        Ok({
            let mut ctx = Self::new();
            let details = serde_json::from_str::<BTreeMap<String, serde_json::Value>>(json)
                .map_err(|err| cdumay_error_json::JsonErrorConverter::convert_error(&err, Some("Failed to load context".to_string()), ctx.inner()))?
                .into_iter()
                .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                .collect();
            ctx.extend(details);
            ctx
        })
    }

    /// Serializes the context to a JSON string.
    ///
    /// This method is only available when the "json" feature is enabled.
    ///
    /// # Parameters
    ///
    /// * `pretty` - If true, the output will be pretty-printed with proper indentation
    ///
    /// # Returns
    ///
    /// Returns `Result<String, Error>` which is:
    /// * `Ok(string)` containing the JSON string on success
    /// * `Err(e)` containing the error on failure
    #[cfg(feature = "json")]
    fn to_json(&self, pretty: bool) -> Result<String, Error> {
        match pretty {
            true => Ok(serde_json::to_string_pretty(&self.inner()).map_err(|err| {
                cdumay_error_json::JsonErrorConverter::convert_error(&err, Some("Failed to dump context".to_string()), self.inner())
            })?),
            false => Ok(serde_json::to_string(&self.inner()).map_err(|err| {
                cdumay_error_json::JsonErrorConverter::convert_error(&err, Some("Failed to dump context".to_string()), self.inner())
            })?),
        }
    }

    /// Creates a new context from a TOML string.
    ///
    /// This method is only available when the "toml" feature is enabled.
    ///
    /// # Parameters
    ///
    /// * `toml` - A string containing valid TOML data
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, Error>` which is:
    /// * `Ok(context)` containing the parsed context on success
    /// * `Err(e)` containing the error on failure
    #[cfg(feature = "toml")]
    fn from_toml(toml: &str) -> Result<Self, Error> {
        Ok({
            let mut ctx = Self::new();
            ctx.extend({
                toml::from_str::<BTreeMap<String, serde_value::Value>>(toml)
                    .map_err(|err| {
                        cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, Some("Failed to load context".to_string()), ctx.inner())
                    })?
                    .into_iter()
                    .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                    .collect()
            });
            ctx
        })
    }

    /// Serializes the context to a TOML string.
    ///
    /// This method is only available when the "toml" feature is enabled.
    ///
    /// # Parameters
    ///
    /// * `pretty` - If true, the output will be pretty-printed with proper indentation
    ///
    /// # Returns
    ///
    /// Returns `Result<String, Error>` which is:
    /// * `Ok(string)` containing the TOML string on success
    /// * `Err(e)` containing the error on failure
    #[cfg(feature = "toml")]
    fn to_toml(&self, pretty: bool) -> Result<String, Error> {
        match pretty {
            true => Ok(toml::to_string_pretty(&self.inner()).map_err(|err| {
                cdumay_error_toml::TomlSerializeErrorConverter::convert_error(&err, Some("Failed to dump context".to_string()), self.inner())
            })?),
            false => Ok(toml::to_string(&self.inner()).map_err(|err| {
                cdumay_error_toml::TomlSerializeErrorConverter::convert_error(&err, Some("Failed to dump context".to_string()), self.inner())
            })?),
        }
    }

    /// Creates a new context from a YAML string.
    ///
    /// This method is only available when the "yaml" feature is enabled.
    ///
    /// # Parameters
    ///
    /// * `yaml` - A string containing valid YAML data
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, Error>` which is:
    /// * `Ok(context)` containing the parsed context on success
    /// * `Err(e)` containing the error on failure
    #[cfg(feature = "yaml")]
    fn from_yaml(yaml: &str) -> Result<Self, Error> {
        Ok({
            let mut ctx = Self::new();
            ctx.extend({
                serde_yaml::from_str::<BTreeMap<String, serde_json::Value>>(yaml)
                    .map_err(|err| {
                        cdumay_error_yaml::YamlErrorConverter::convert_error(&err, Some("Failed to load context".to_string()), ctx.inner())
                    })?
                    .into_iter()
                    .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                    .collect()
            });
            ctx
        })
    }

    /// Serializes the context to a YAML string.
    ///
    /// This method is only available when the "yaml" feature is enabled.
    ///
    /// # Returns
    ///
    /// Returns `Result<String, Error>` which is:
    /// * `Ok(string)` containing the YAML string on success
    /// * `Err(e)` containing the error on failure
    #[cfg(feature = "yaml")]
    fn to_yaml(&self) -> Result<String, Error> {
        Ok(serde_yaml::to_string(&self.inner())
            .map_err(|err| cdumay_error_yaml::YamlErrorConverter::convert_error(&err, Some("Failed to dump context".to_string()), self.inner()))?)
    }

}

/// A dynamic key-value context container that can store heterogeneous data.
///
/// Internally uses a `BTreeMap<String, serde_value::Value>`, allowing you
/// to insert any serializable value while preserving insertion order and
/// allowing serialization/deserialization.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Context {
    /// The internal map storing the context data.
    data: BTreeMap<String, serde_value::Value>,
}

impl Contextualize for Context {
    /// Creates a new, empty `Context`.
    fn new() -> Self {
        Self::default()
    }

    /// Inserts a key-value pair into the context.
    ///
    /// # Arguments
    /// * `k` - The key as a `String`.
    /// * `v` - The value as a `serde_value::Value`.
    fn insert(&mut self, k: String, v: serde_value::Value) {
        self.data.insert(k, v);
    }

    /// Retrieves a reference to a value associated with the given key.
    ///
    /// # Arguments
    /// * `k` - The key as a string slice.
    ///
    /// # Returns
    /// * `Some(&Value)` if the key exists, or `None` otherwise.
    fn get(&self, k: &str) -> Option<&serde_value::Value> {
        self.data.get(k)
    }

    /// Extends the context with the given key-value pairs.
    ///
    /// Existing keys will be overwritten.
    ///
    /// # Arguments
    /// * `data` - A `BTreeMap` of key-value pairs to insert.
    fn extend(&mut self, data: BTreeMap<String, serde_value::Value>) {
        self.data.extend(data);
    }

    /// Returns a cloned copy of the internal map.
    ///
    /// Useful for inspection or when you need owned data.
    fn inner(&self) -> BTreeMap<String, serde_value::Value> {
        self.data.clone()
    }
}
