use crate::{Context, Error};
use serde::Deserialize;
use std::collections::BTreeMap;

impl From<serde_yaml::Error> for Error {
    /// Converts a `serde_yaml::Error` (YAML serialization/deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `serde_yaml::Error` into `Error::Yaml(String)`,
    /// making it easier to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```
    /// fn parse_yaml<T: serde::de::DeserializeOwned>(yaml_str: &str) -> cdumay_context::Result<T> {
    ///     Ok(serde_yaml::from_str(yaml_str)?) // `?` converts serde_yaml::Error into Error::Yaml
    /// }
    ///
    /// let yaml_str = "name: Alice\nage: 30";
    /// let result: cdumay_context::Result<()> = parse_yaml::<()>(yaml_str);
    /// assert!(result.is_err()); // Example case where parsing might fail
    /// ```
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err.to_string())
    }
}

impl Context {
    /// Creates a `Context` from a YAML string.
    ///
    /// This function attempts to parse the input YAML into a `BTreeMap<String, serde_json::Value>`,
    /// then converts each value into `serde_value::Value` using `.deserialize()`.
    ///
    /// # Errors
    /// - Returns an `Error::Yaml` variant if YAML parsing fails.
    /// - Panics if deserialization of `serde_json::Value` to `serde_value::Value` fails (due to `.unwrap()`).
    ///
    /// # Example
    /// ```
    /// let yaml_str = "name: Alice\nage: 30";
    /// let context = cdumay_context::Context::from_yaml(yaml_str).unwrap();
    ///
    /// assert_eq!(context.get("name").unwrap(), &serde_value::Value::String("Alice".to_string()));
    /// assert_eq!(context.get("age").unwrap(), &serde_value::Value::U64(30));
    /// ```
    pub fn from_yaml(yaml: &str) -> crate::Result<Context> {
        Ok(Context {
            inner: serde_yaml::from_str::<BTreeMap<String, serde_json::Value>>(yaml)?
                .into_iter()
                .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                .collect(),
        })
    }

    /// Serializes the `Context` into a YAML string.
    ///
    /// # Errors
    /// - Returns an `Error::Yaml` variant if serialization fails.
    ///
    /// # Example
    /// ```
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
    ///
    /// let yaml = context.to_yaml().unwrap();
    /// println!("{}", yaml); // Serialized YAML output
    /// ```
    pub fn to_yaml(&self) -> crate::Result<String> {
        Ok(serde_yaml::to_string(&self)?)
    }
}
