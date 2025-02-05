use crate::{Context, Error};
use serde::Deserialize;
use std::collections::BTreeMap;

impl From<serde_json::Error> for Error {
    /// Converts a `serde_json::Error` into the `cdumay_context::Error` enum.
    ///
    /// This allows automatic conversion of `serde_json::Error` into `Error::Json(String)`
    /// when using the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// Example:
    /// ```rust
    /// fn parse_json(input: &str) -> cdumay_context::Result<serde_json::Value> {
    ///     let value: serde_json::Value = serde_json::from_str(input)?; // Automatically converts serde_json::Error into Error
    ///     Ok(value)
    /// }
    ///
    /// let json_str = "{ invalid_json }";
    /// let result = parse_json(json_str);
    ///
    /// assert!(matches!(result, Err(cdumay_context::Error::Json(_))));
    /// ```
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl Context {
    /// Creates a `Context` from a JSON string.
    ///
    /// This function attempts to parse the input JSON into a `BTreeMap<String, serde_json::Value>`,
    /// then converts each value into `serde_value::Value` using `.deserialize()`.
    ///
    /// # Errors
    /// - Returns an `Error::Json` variant if the JSON parsing fails.
    /// - Panics if deserialization of `serde_json::Value` to `serde_value::Value` fails (use `.unwrap()`).
    ///
    /// # Example
    /// ```rust
    /// let json_str = r#"{"name": "Alice", "age": 30}"#;
    /// let context = cdumay_context::Context::from_json(json_str).unwrap();
    ///
    /// assert_eq!(context.get("name").unwrap(), &serde_value::Value::String("Alice".to_string()));
    /// assert_eq!(context.get("age").unwrap(), &serde_value::Value::U64(30));
    /// ```
    pub fn from_json(json: &str) -> crate::Result<Context> {
        Ok(Context {
            inner: serde_json::from_str::<BTreeMap<String, serde_json::Value>>(json)?
                .into_iter()
                .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                .collect(),
        })
    }

    /// Serializes the `Context` into a JSON string.
    ///
    /// # Parameters
    /// - `pretty`: If `true`, returns formatted JSON with indentation.
    /// - If `false`, returns compact JSON without extra spaces.
    ///
    /// # Errors
    /// - Returns an `Error::Json` variant if serialization fails.
    ///
    /// # Example
    /// ```rust
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
    ///
    /// let json = context.to_json(true).unwrap();
    /// println!("{}", json); // Pretty-printed JSON
    ///
    /// let compact_json = context.to_json(false).unwrap();
    /// println!("{}", compact_json); // Minified JSON
    /// ```
    pub fn to_json(&self, pretty: bool) -> crate::Result<String> {
        match pretty {
            true => Ok(serde_json::to_string_pretty(self)?),
            false => Ok(serde_json::to_string(self)?),
        }
    }
}
