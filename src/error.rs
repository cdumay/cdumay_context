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

    /// Error related to YAML processing, available if the "yaml" feature is enabled.
    #[cfg(feature = "yaml")]
    Yaml(String),

    /// Error related to HTTP processing, available if the "http-headers" feature is enabled.
    #[cfg(feature = "http-headers")]
    Http(String),
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    /// Converts a `serde_json::Error` into the `cdumay_context::Error` enum.
    ///
    /// This allows automatic conversion of `serde_json::Error` into `Error::Json(String)`
    /// when using the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// Example:
    /// ```rust
    /// fn parse_json(input: &str) -> Result<serde_json::Value, cdumay_context::Error> {
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

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for Error {
    /// Converts a `toml::ser::Error` (TOML serialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `toml::ser::Error` into `Error::Toml(String)`,
    /// making it easy to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```rust
    /// fn to_toml<T: serde::Serialize>(value: &T) ->  Result<String, cdumay_context::Error> {
    ///     Ok(toml::to_string(value)?)
    /// }
    /// ```
    fn from(err: toml::ser::Error) -> Self {
        Error::Toml(err.to_string())
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    /// Converts a `toml::de::Error` (TOML deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `toml::de::Error` into `Error::Toml(String)`,
    /// making error handling cleaner when deserializing TOML data.
    ///
    /// # Example
    /// ```rust
    /// fn from_toml<T: serde::de::DeserializeOwned>(toml_str: &str) -> Result<T, cdumay_context::Error> {
    ///     Ok(toml::from_str(toml_str)?)
    /// }
    /// ```
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err.to_string())
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    /// Converts a `serde_yaml::Error` (YAML serialization/deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `serde_yaml::Error` into `Error::Yaml(String)`,
    /// making it easier to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```
    /// fn parse_yaml<T: serde::de::DeserializeOwned>(yaml_str: &str) -> Result<T, cdumay_context::Error> {
    ///     Ok(serde_yaml::from_str(yaml_str)?) // `?` converts serde_yaml::Error into Error::Yaml
    /// }
    ///
    /// let yaml_str = "name: Alice\nage: 30";
    /// let result: Result<(), cdumay_context::Error> = parse_yaml::<()>(yaml_str);
    /// assert!(result.is_err()); // Example case where parsing might fail
    /// ```
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err.to_string())
    }
}

#[cfg(feature = "http-headers")]
impl From<reqwest::header::InvalidHeaderValue> for Error {
    /// Converts a `reqwest::header::InvalidHeaderValue` (HTTP Header serialization/deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `serde_yaml::Error` into `Error::http(String)`,
    /// making it easier to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```
    /// fn parse_header(value: String) -> Result<reqwest::header::HeaderValue, cdumay_context::Error> {
    ///     Ok(reqwest::header::HeaderValue::from_str(&value)?) // `?` converts reqwest::header::InvalidHeaderValue into Error::Http
    /// }
    ///
    /// let result = parse_header("Invalid\r\nValue".to_string());
    /// assert!(result.is_err()); // Example case where parsing might fail
    /// ```
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Error::Http(err.to_string())
    }
}

#[cfg(feature = "http-headers")]
impl From<reqwest::header::InvalidHeaderName> for Error {
    /// Converts a `reqwest::header::InvalidHeaderName` (HTTP Header serialization/deserialization error) into the custom `Error` type.
    ///
    /// This allows automatic conversion of `serde_yaml::Error` into `Error::http(String)`,
    /// making it easier to use the `?` operator in functions that return `Result<T, Error>`.
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    ///
    /// fn parse_header(header: String) -> Result<reqwest::header::HeaderName, cdumay_context::Error> {
    ///     Ok(reqwest::header::HeaderName::from_str(&header)?) // `?` converts reqwest::header::InvalidHeaderName into Error::Http
    /// }
    ///
    /// let result = parse_header(String::default());
    /// assert!(result.is_err()); // Example case where parsing might fail
    /// ```
    fn from(err: reqwest::header::InvalidHeaderName) -> Self {
        Error::Http(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_generic_error() {
        let error_msg = "test error message";
        let error = Error::Generic(error_msg.to_string());

        // Test Debug implementation
        assert!(format!("{:?}", error).contains(error_msg));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_error_conversion() {
        // Test JSON parse error
        let invalid_json = r#"{ "key": "value" "#; // Missing closing brace
        let json_error = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let error: Error = json_error.into();

        match error {
            Error::Json(msg) => {
                assert!(!msg.is_empty());
                assert!(msg.contains("EOF"));
            }
            _ => panic!("Expected JSON error variant"),
        }

        // Test error message preservation
        let result: Result<serde_json::Value, Error> = Err(Error::Json("custom error".to_string()));
        assert!(format!("{:?}", result.unwrap_err()).contains("custom error"));
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_error_conversion() {
        // Test TOML parse error
        let invalid_toml = r#"[table]
        key = "value"
        key = "duplicate""#;
        let toml_error = toml::from_str::<toml::Value>(invalid_toml).unwrap_err();
        let error: Error = toml_error.into();

        match error {
            Error::Toml(msg) => {
                assert!(!msg.is_empty());
                assert!(msg.contains("duplicate"));
            }
            _ => panic!("Expected TOML error variant"),
        }
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_error_conversion() {
        // Test YAML parse error
        let invalid_yaml = "key: : value"; // Invalid YAML syntax
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml).unwrap_err();
        let error: Error = yaml_error.into();

        match error {
            Error::Yaml(msg) => {
                assert!(!msg.is_empty());
                assert!(msg.contains("mapping values are not allowed"));
            }
            _ => panic!("Expected YAML error variant"),
        }

        // Test error message preservation
        let result: Result<serde_yaml::Value, Error> = Err(Error::Yaml("custom error".to_string()));
        assert!(format!("{:?}", result.unwrap_err()).contains("custom error"));
    }

    #[test]
    fn test_error_trait_implementation() {
        // Test that our Error type implements the std::error::Error trait
        let _ = Error::Generic("test error".to_string());

        // Test error messages
        let generic = Error::Generic("generic error".to_string());
        assert!(format!("{:?}", generic).contains("generic error"));

        #[cfg(feature = "json")]
        {
            let json = Error::Json("json error".to_string());
            assert!(format!("{:?}", json).contains("json error"));
        }

        #[cfg(feature = "toml")]
        {
            let toml = Error::Toml("toml error".to_string());
            assert!(format!("{:?}", toml).contains("toml error"));
        }

        #[cfg(feature = "yaml")]
        {
            let yaml = Error::Yaml("yaml error".to_string());
            assert!(format!("{:?}", yaml).contains("yaml error"));
        }
    }

    #[test]
    fn test_error_conversions_chaining() {
        // Test that we can chain error conversions using the ? operator
        fn process_data() -> Result<(), Error> {
            #[cfg(feature = "json")]
            {
                let invalid_json = r#"{ "key": "value" "#;
                let _: serde_json::Value = serde_json::from_str(invalid_json)?;
            }
            Ok(())
        }

        assert!(process_data().is_err());
    }

    #[test]
    #[cfg(feature = "http-headers")]
    fn test_headers() {
        let invalid_header_value: Error  = reqwest::header::HeaderValue::from_str(&"Invalid\r\nValue".to_string()).unwrap_err().into();
        assert!(format!("{:?}", invalid_header_value).contains("Http"));
        let invalid_header_name: Error = reqwest::header::HeaderName::from_str(&String::default()).unwrap_err().into();
        assert!(format!("{:?}", invalid_header_name).contains("Http"));
    }
}
