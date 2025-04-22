#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use serde_value::Value;
    use cdumay_context::{Context, Contextualize};

    #[test]
    fn test_new() {
        let ctx = Context::new();
        assert!(ctx.inner().is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut ctx = Context::new();

        // Test string value
        ctx.insert("string_key".to_string(), Value::String("test value".to_string()));
        assert_eq!(ctx.get("string_key").unwrap(), &Value::String("test value".to_string()));

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
        ctx.insert("string_key".to_string(), Value::String("new value".to_string()));
        assert_eq!(ctx.get("string_key").unwrap(), &Value::String("new value".to_string()));
    }

    #[test]
    fn test_extend() {
        let mut ctx = Context::new();
        let mut data = BTreeMap::new();

        data.insert("key1".to_string(), Value::String("value1".to_string()));
        data.insert("key2".to_string(), Value::I64(42));

        ctx.extend(data);

        assert_eq!(ctx.get("key1").unwrap(), &Value::String("value1".to_string()));
        assert_eq!(ctx.get("key2").unwrap(), &Value::I64(42));

        // Test extending with overlapping keys
        let mut new_data = BTreeMap::new();
        new_data.insert("key1".to_string(), Value::String("new value".to_string()));
        new_data.insert("key3".to_string(), Value::Bool(true));

        ctx.extend(new_data);

        assert_eq!(ctx.get("key1").unwrap(), &Value::String("new value".to_string()));
        assert_eq!(ctx.get("key2").unwrap(), &Value::I64(42));
        assert_eq!(ctx.get("key3").unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_inner() {
        let mut ctx = Context::new();

        ctx.insert("key1".to_string(), Value::String("value1".to_string()));
        ctx.insert("key2".to_string(), Value::I64(42));

        let inner_data = ctx.inner();
        assert_eq!(inner_data.len(), 2);
        assert_eq!(inner_data.get("key1").unwrap(), &Value::String("value1".to_string()));
        assert_eq!(inner_data.get("key2").unwrap(), &Value::I64(42));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        let mut ctx = Context::new();
        ctx.insert("string".to_string(), Value::String("test".to_string()));
        ctx.insert("number".to_string(), Value::U64(42));
        ctx.insert("boolean".to_string(), Value::Bool(true));

        // Test to_json
        let json = ctx.to_json(false).unwrap();
        let parsed: BTreeMap<String, Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get("string").unwrap().clone(), Value::String("test".to_string()));
        assert_eq!(parsed["number"], Value::U64(42));
        assert_eq!(parsed["boolean"], Value::Bool(true));

        // Test from_json
        let ctx2 = Context::from_json(&json).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test pretty printing
        let pretty_json = ctx.to_json(true).unwrap();
        assert!(pretty_json.contains("\n"));

        // Test invalid JSON
        assert!(Context::from_json("invalid json").is_err());
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_serialization() {
        let mut ctx = Context::new();
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
        let ctx2 = Context::from_toml(&toml_str).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test pretty printing
        let pretty_toml = ctx.to_toml(true).unwrap();
        assert!(pretty_toml.contains("\n"));

        // Test invalid TOML
        assert!(Context::from_toml("invalid = toml").is_err());
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_serialization() {
        let mut ctx = Context::new();
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
        let ctx2 = Context::from_yaml(&yaml).unwrap();
        assert_eq!(ctx.inner(), ctx2.inner());

        // Test invalid YAML
        assert!(Context::from_yaml("invalid: - yaml: ]").is_err());
    }
}
