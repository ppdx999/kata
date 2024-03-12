use serde_json;
use serde_json::{Map, Value};
use crate::json::data as schema;

#[allow(dead_code)]
pub struct Validator;

impl Validator {
    pub fn validate(schema: &schema::Value, text: &str) -> Result<bool, String> {
        let value: serde_json::Value = serde_json::from_str(text).map_err( |e| e.to_string() )?;
        match value {
            Value::Object(object) => match schema {
                schema::Value::Object(schema) => Self::object(&schema, &object),
            },
            _ => Err("Expected an object".to_string()),
        }
    }

    fn object(schema: &schema::Object, object: &Map<String, Value>) -> Result<bool, String> {
        for property in &schema.properties {
            let value = object.get(&property.name).ok_or(format!("Property '{}' not found", property.name))?;
            Self::property(&property, value)?;
        }
        Ok(true)
    }

    fn property(schema: &schema::Property, value: &Value) -> Result<bool, String> {
        match &schema.type_ {
            schema::Type::String => match value {
                Value::String(_) => Ok(true),
                _ => Err("Expected a string".to_string()),
            },
            schema::Type::Object(schema) => match value {
                Value::Object(object) => Self::object(schema, object),
                _ => Err("Expected an object".to_string()),
            },
        }
    }
}

#[test]
fn test_empty_object() {
    use crate::json::schema::Schema;

    let schema = Schema::from_text("{}").unwrap();
    assert!(Validator::validate(&schema.root, "{}").unwrap());
}

#[test]
fn test_simple_object() {
    use crate::json::schema::Schema;

    let schema = Schema::from_text("{name: string}").unwrap();

    assert!(Validator::validate(&schema.root, r#"{"name": "John"}"#).unwrap());

    assert_eq!(Validator::validate(&schema.root, r#"{"nome": "John"}"#).unwrap_err(), "Property 'name' not found");
}
