use serde_json;
use serde_json::{Map, Value};
use crate::json::schema::Schema;
use crate::json::data::Node;

#[allow(dead_code)]
pub struct Validator;

impl Validator {
    // use root node instead of schema
    pub fn validate(schema: &Schema, text: &str) -> Result<bool, String> {
        let value: serde_json::Value = serde_json::from_str(text).map_err( |e| e.to_string() )?;
        match value {
            Value::Object(object) => Self::object(&schema.root, object),
            _ => Err("Expected an object".to_string()),
        }
    }

    fn object(schema: &Node, object: Map<String, Value>) -> Result<bool, String> {
        Ok(true)
    }
}

#[test]
fn test_validator() {
    let schema = Schema::from_text("{}").unwrap();
    assert!(Validator::validate(&schema, "{}").unwrap());
}

