//! Context management trait and implementations.
//!
//! This module provides the [`Context`] trait, which defines a generic interface for
//! managing key-value data with support for various serialization formats.

use serde::Serialize;
use std::collections::BTreeMap;
#[cfg(any(feature = "json", feature = "toml", feature = "yaml"))]
use serde::Deserialize;

/// A trait for managing key-value context data with serialization support.
///
/// The `Context` trait provides a standardized interface for managing key-value data
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
/// use serde::{Serialize, Deserialize};
/// use cdumay_context::Context;
///
/// #[derive(Default, Serialize, Deserialize)]
/// struct MyContext {
///     data: BTreeMap<String, serde_value::Value>
/// }
///
/// impl Context for MyContext {
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
pub trait Context: Sized + Serialize {
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
    /// # use cdumay_context::Context;
    /// # #[cfg(feature = "json")]
    /// # fn example<T: Context>(json: &str) -> Result<T, cdumay_context::Error> {
    /// let ctx = T::from_json(json)?;
    /// # Ok(ctx)
    /// # }
    /// ```
    #[cfg(feature = "json")]
    fn from_json(json: &str) -> Result<Self, crate::Error> {
        Ok({
            let mut ctx = Self::new();
            ctx.extend({
                serde_json::from_str::<BTreeMap<String, serde_json::Value>>(json)?
                    .into_iter()
                    .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                    .collect()
            });
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
    fn to_json(&self, pretty: bool) -> Result<String, crate::Error> {
        match pretty {
            true => Ok(serde_json::to_string_pretty(&self.inner())?),
            false => Ok(serde_json::to_string(&self.inner())?),
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
    fn from_toml(toml: &str) -> Result<Self, crate::Error> {
        Ok({
            let mut ctx = Self::new();
            ctx.extend({
                toml::from_str::<BTreeMap<String, serde_value::Value>>(toml)?
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
    fn to_toml(&self, pretty: bool) -> Result<String, crate::Error> {
        match pretty {
            true => Ok(toml::to_string_pretty(&self.inner())?),
            false => Ok(toml::to_string(&self.inner())?),
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
    fn from_yaml(yaml: &str) -> Result<Self, crate::Error> {
        Ok({
            let mut ctx = Self::new();
            ctx.extend({
                serde_yaml::from_str::<BTreeMap<String, serde_json::Value>>(yaml)?
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
    fn to_yaml(&self) -> Result<String, crate::Error> {
        Ok(serde_yaml::to_string(&self.inner())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_value::Value;

    #[derive(Default, Serialize, Deserialize, Debug)]
    struct TestContext {
        data: BTreeMap<String, Value>,
    }

    impl Context for TestContext {
        fn new() -> Self {
            Self::default()
        }

        fn insert(&mut self, k: String, v: Value) {
            self.data.insert(k, v);
        }

        fn get(&self, k: &str) -> Option<&Value> {
            self.data.get(k)
        }

        fn extend(&mut self, data: BTreeMap<String, Value>) {
            self.data.extend(data);
        }

        fn inner(&self) -> BTreeMap<String, Value> {
            self.data.clone()
        }
    }

    #[test]
    fn test_new() {
        let ctx = TestContext::new();
        assert!(ctx.inner().is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut ctx = TestContext::new();

        // Test string value
        ctx.insert(
            "string_key".to_string(),
            Value::String("test value".to_string()),
        );
        assert_eq!(
            ctx.get("string_key").unwrap(),
            &Value::String("test value".to_string())
        );

        // Test integer value
        ctx.insert("int_key".to_string(), Value::I64(42));
        assert_eq!(ctx.get("int_key").unwrap(), &Value::I64(42));

        // Test float value
        ctx.insert("float_key".to_string(), Value::F64(3.14));
        assert_eq!(ctx.get("float_key").unwrap(), &Value::F64(3.14));

        // Test boolean value
        ctx.insert("bool_key".to_string(), Value::Bool(true));
        assert_eq!(ctx.get("bool_key").unwrap(), &Value::Bool(true));

        // Test non-existent key
        assert!(ctx.get("non_existent").is_none());

        // Test overwriting existing key
        ctx.insert(
            "string_key".to_string(),
            Value::String("new value".to_string()),
        );
        assert_eq!(
            ctx.get("string_key").unwrap(),
            &Value::String("new value".to_string())
        );
    }

    #[test]
    fn test_extend() {
        let mut ctx = TestContext::new();
        let mut data = BTreeMap::new();

        data.insert("key1".to_string(), Value::String("value1".to_string()));
        data.insert("key2".to_string(), Value::I64(42));

        ctx.extend(data);

        assert_eq!(
            ctx.get("key1").unwrap(),
            &Value::String("value1".to_string())
        );
        assert_eq!(ctx.get("key2").unwrap(), &Value::I64(42));

        // Test extending with overlapping keys
        let mut new_data = BTreeMap::new();
        new_data.insert("key1".to_string(), Value::String("new value".to_string()));
        new_data.insert("key3".to_string(), Value::Bool(true));

        ctx.extend(new_data);

        assert_eq!(
            ctx.get("key1").unwrap(),
            &Value::String("new value".to_string())
        );
        assert_eq!(ctx.get("key2").unwrap(), &Value::I64(42));
        assert_eq!(ctx.get("key3").unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_inner() {
        let mut ctx = TestContext::new();

        ctx.insert("key1".to_string(), Value::String("value1".to_string()));
        ctx.insert("key2".to_string(), Value::I64(42));

        let inner_data = ctx.inner();
        assert_eq!(inner_data.len(), 2);
        assert_eq!(
            inner_data.get("key1").unwrap(),
            &Value::String("value1".to_string())
        );
        assert_eq!(inner_data.get("key2").unwrap(), &Value::I64(42));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        let mut ctx = TestContext::new();
        ctx.insert("string".to_string(), Value::String("test".to_string()));
        ctx.insert("number".to_string(), Value::U64(42));
        ctx.insert("boolean".to_string(), Value::Bool(true));

        // Test to_json
        let json = ctx.to_json(false).unwrap();
        let parsed: BTreeMap<String, Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.get("string").unwrap().clone(),
            Value::String("test".to_string())
        );
        assert_eq!(parsed["number"], Value::U64(42));
        assert_eq!(parsed["boolean"], Value::Bool(true));

        // Test from_json
        let ctx2 = TestContext::from_json(&json).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test pretty printing
        let pretty_json = ctx.to_json(true).unwrap();
        assert!(pretty_json.contains("\n"));

        // Test invalid JSON
        assert!(TestContext::from_json("invalid json").is_err());
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_serialization() {
        let mut ctx = TestContext::new();
        ctx.insert("string".to_string(), Value::String("test".to_string()));
        ctx.insert("number".to_string(), Value::I64(42));
        ctx.insert("boolean".to_string(), Value::Bool(true));

        // Test to_toml
        let toml_str = ctx.to_toml(false).unwrap();
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed["string"].as_str().unwrap(), "test");
        assert_eq!(parsed["number"].as_integer().unwrap(), 42);
        assert_eq!(parsed["boolean"].as_bool().unwrap(), true);

        // Test from_toml
        let ctx2 = TestContext::from_toml(&toml_str).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test pretty printing
        let pretty_toml = ctx.to_toml(true).unwrap();
        assert!(pretty_toml.contains("\n"));

        // Test invalid TOML
        assert!(TestContext::from_toml("invalid = toml").is_err());
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_serialization() {
        let mut ctx = TestContext::new();
        ctx.insert("string".to_string(), Value::String("test".to_string()));
        ctx.insert("number".to_string(), Value::U64(42));
        ctx.insert("boolean".to_string(), Value::Bool(true));

        // Test to_yaml
        let yaml = ctx.to_yaml().unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed["string"].as_str().unwrap(), "test");
        assert_eq!(parsed["number"].as_i64().unwrap(), 42);
        assert_eq!(parsed["boolean"].as_bool().unwrap(), true);

        // Test from_yaml
        let ctx2 = TestContext::from_yaml(&yaml).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test invalid YAML
        assert!(TestContext::from_yaml("invalid: - yaml: ]").is_err());
    }
}
