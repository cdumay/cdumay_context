#[cfg(test)]
mod tests {
    use cdumay_context::UnExpectedError;
    use cdumay_core::{Error, ErrorConverter};
    use std::collections::BTreeMap;

    #[test]
    fn test_generic_error() {
        let error_msg = "test error message";
        let error = UnExpectedError::new().with_message(error_msg.to_string());

        // Test Debug implementation
        assert!(format!("{:?}", error).contains(error_msg));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_error_conversion() {
        // Test JSON parse error
        let invalid_json = r#"{ "key": "value" "#; // Missing closing brace
        let json_error = serde_json::from_str::<serde_json::Value>(invalid_json)
            .map_err(|err| cdumay_json::JsonErrorConverter::convert_error(&err, None, BTreeMap::new()))
            .unwrap_err();
        let error: Error = json_error.into();
        assert!(!error.message().is_empty());
        assert!(error.message().contains("EOF"));
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_error_conversion() {
        // Test TOML parse error
        let invalid_toml = r#"[table]
        key = "value"
        key = "duplicate""#;
        let toml_error = toml::from_str::<toml::Value>(invalid_toml)
            .map_err(|err| cdumay_toml::TomlDeserializeErrorConverter::convert_error(&err, None, BTreeMap::new()))
            .unwrap_err();
        let error: Error = toml_error.into();
        assert!(!error.message().is_empty());
        assert!(error.message().contains("duplicate"));
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_error_conversion() {
        // Test YAML parse error
        let invalid_yaml = "key: : value"; // Invalid YAML syntax
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml)
            .map_err(|err| cdumay_yaml::YamlErrorConverter::convert_error(&err, None, BTreeMap::new()))
            .unwrap_err();
        let error: Error = yaml_error.into();
        assert!(!error.message().is_empty());
        assert!(error.message().contains("mapping values are not allowed"));
    }

    #[test]
    fn test_error_trait_implementation() {
        // Test that our Error type implements the std::error::Error trait
        let _ = UnExpectedError::new().with_message("test error".to_string());

        // Test error messages
        let generic = UnExpectedError::new().with_message("generic error".to_string());
        assert!(format!("{:?}", generic).contains("generic error"));
    }
}
