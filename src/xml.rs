use crate::{Context, Error};
use serde::Deserialize;
use std::collections::BTreeMap;

impl From<serde_xml_rs::Error> for Error {
    /// Converts a `serde_xml_rs::Error` (XML serialization/deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `serde_xml_rs::Error` into `Error::Xml(String)`,
    /// making it easier to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```rust
    /// fn parse_xml<T: serde::de::DeserializeOwned>(xml_str: &str) -> cdumay_context::Result<T> {
    ///     Ok(serde_xml_rs::from_str(xml_str)?) // `?` converts serde_xml_rs::Error into Error::Xml
    /// }
    ///
    /// let xml_str = r#"<person><name>Alice</name></person>"#;
    /// let result: cdumay_context::Result<()> = parse_xml::<()>(xml_str);
    /// assert!(result.is_err()); // Example case where parsing might fail
    /// ```
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::Xml(err.to_string())
    }
}


impl Context {
    /// Creates a `Context` from an XML string.
    ///
    /// This function attempts to parse the input XML into a `BTreeMap<String, serde_json::Value>`,
    /// then converts each value into `serde_value::Value` using `.deserialize()`.
    ///
    /// # Errors
    /// - Returns an `Error::Xml` variant if XML parsing fails.
    /// - Panics if deserialization of `serde_json::Value` to `serde_value::Value` fails (due to `.unwrap()`).
    ///
    /// # Example
    /// ```rust
    ///     let xml_str = r#"<root><name>Alice</name><age>30</age></root>"#;
    ///     let context = cdumay_context::Context::from_xml(xml_str).unwrap();
    ///
    ///
    ///     let name : std::collections::BTreeMap<String, String> = context.get("name").unwrap().clone().deserialize_into().unwrap();
    ///     let age : std::collections::BTreeMap<String, String> = context.get("age").unwrap().clone().deserialize_into().unwrap();
    ///
    ///     assert_eq!(name.get("$value").unwrap(), "Alice");
    ///     assert_eq!(age.get("$value").unwrap(), "30");
    /// ```
    pub fn from_xml(xml: &str) -> crate::Result<Context> {
        Ok(Context {
            inner: serde_xml_rs::from_str::<BTreeMap<String, serde_value::Value>>(xml)?
                .into_iter()
                .map(|(key, value)| (key, serde_value::Value::deserialize(value).unwrap()))
                .collect(),
        })
    }

    /// Serializes the `Context` into an XML string.
    ///
    /// # Errors
    /// - Returns an `Error::Xml` variant if serialization fails.
    ///
    /// # Example
    /// ```rust
    /// let mut context = cdumay_context::Context::new();
    /// context.insert("name".to_string(), serde_value::Value::String("Alice".to_string()));
    ///
    /// let xml = context.to_xml().unwrap();
    /// println!("{}", xml); // Serialized XML output
    /// ```
    pub fn to_xml(&self) -> crate::Result<String> {
        Ok(serde_xml_rs::to_string(&self)?)
    }
}
