use crate::{Context, Error};
use serde::Deserialize;
use std::collections::BTreeMap;

impl From<toml::ser::Error> for Error {
    /// Converts a `toml::ser::Error` (TOML serialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `toml::ser::Error` into `Error::Toml(String)`,
    /// making it easy to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```rust
    /// fn to_toml<T: serde::Serialize>(value: &T) -> cdumay_context::Result<String> {
    ///     Ok(toml::to_string(value)?)
    /// }
    /// ```
    fn from(err: toml::ser::Error) -> Self {
        Error::Toml(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    /// Converts a `toml::de::Error` (TOML deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `toml::de::Error` into `Error::Toml(String)`,
    /// making error handling cleaner when deserializing TOML data.
    ///
    /// # Example
    /// ```rust
    /// fn from_toml<T: serde::de::DeserializeOwned>(toml_str: &str) -> cdumay_context::Result<T> {
    ///     Ok(toml::from_str(toml_str)?)
    /// }
    /// ```
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err.to_string())
    }
}


impl Context {
    /// Creates a `Context` from a TOML string.
    ///
    /// This function attempts to parse the input TOML into a `BTreeMap<String, serde_json::Value>`,
    /// then converts each value into `serde_value::Value` using `.deserialize()`.
    ///
    /// # Errors
    /// - Returns an `Error::Toml` variant if the TOML parsing fails.
    /// - Panics if deserialization of `serde_json::Value` to `serde_value::Value` fails (use `.unwrap()`).
    ///
    /// # Example
    /// ```rust
    ///     let toml_str = r#"
    ///     name = "Alice"
    ///     age = 30
    ///     "#;
    ///     let context = cdumay_context::Context::from_toml(toml_str).unwrap();
    ///
    ///     assert_eq!(context.get("name").unwrap(), &serde_value::Value::String("Alice".to_string()));
    ///     assert_eq!(context.get("age").unwrap(), &serde_value::Value::I64(30));
    /// ```
    pub fn from_toml(toml: &str) -> crate::Result<Context> {
        Ok(Context {
            inner: toml::from_str::<BTreeMap<String, serde_value::Value>>(toml)?
                .into_iter()
                .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                .collect(),
        })
    }

    /// Serializes the `Context` into a TOML string.
    ///
    /// # Parameters
    /// - `pretty`: If `true`, returns formatted TOML with indentation.
    /// - If `false`, returns compact TOML without extra spaces.
    ///
    /// # Errors
    /// - Returns an `Error::Toml` variant if serialization fails.
    ///
    /// # Example
    /// ```rust
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
    ///
    /// let toml = context.to_toml(true).unwrap();
    /// println!("{}", toml); // Pretty-printed TOML
    ///
    /// let compact_toml = context.to_toml(false).unwrap();
    /// println!("{}", compact_toml); // Minified TOML
    /// ```
    pub fn to_toml(&self, pretty: bool) -> crate::Result<String> {
        match pretty {
            true => Ok(toml::to_string_pretty(self)?),
            false => Ok(toml::to_string(&self)?),
        }
    }
}
