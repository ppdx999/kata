use serde_json;
use serde_json::{Map, Value};

use super::data as schema;
use super::error::{ValidationError, ValidationErrors};

#[allow(dead_code)]
pub struct Validator;

impl Validator {
    pub fn validate(schema: &schema::Value, text: &str) -> Result<bool, ValidationErrors> {
        let value: serde_json::Value = Self::parse_json(text)?;

        match value {
            Value::Object(object) => match schema {
                schema::Value::Object(schema) => Self::object(&schema, &object),
            },
            _ => Err(
                ValidationError::ParseFaild(
                    "Top level value must be an object".to_string(),
                ).into()
            )
        }
    }

    fn parse_json(text: &str) -> Result<Value, ValidationError> {
        Ok(serde_json::from_str(text)?)
    }

    fn object(schema: &schema::Object, object: &Map<String, Value>) -> Result<bool, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for property in &schema.properties {
            match object.get(&property.name) {
                Some(value) => {
                    match Self::property(&property, value) {
                        Ok(_) => {},
                        Err(errs) => errors.extend(errs),
                    }
                },
                None => errors.0.push(
                    ValidationError::PropertyNotFound {
                        name: property.name.clone(),
                    }
                ),
            }
        }

        if errors.is_empty() {
            Ok(true)
        } else {
            Err(errors)
        }
    }

    fn property(schema: &schema::Property, value: &Value) -> Result<bool, ValidationErrors> {
        match &schema.type_ {
            schema::Type::String => match value {
                Value::String(_) => Ok(true),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "string".to_string(),
                    value: value.to_string(),
                }.into()),
            },
            schema::Type::Object(schema) => match value {
                Value::Object(object) => Ok(Self::object(schema, object)?),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "object".to_string(),
                    value: value.to_string(),
                }.into()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::schema::Schema;

    #[test]
    fn test_empty_object() {
        let schema = Schema::from_text("{}").unwrap();
        assert!(Validator::validate(&schema.root, "{}").unwrap());
    }

    #[test]
    fn test_single_string() {
        let schema = Schema::from_text("{id: string}").unwrap();
        assert!(Validator::validate(&schema.root, r#"{"id": "xxx-yyy"}"#).unwrap());
        assert_eq!(
            Validator::validate(&schema.root, r#"{"id":0}"#).unwrap_err(),
            ValidationErrors(vec![ValidationError::DataTypeMismatch {
                type_: "string".to_string(),
                value: "0".to_string()
            }])
        )
    }

    #[test]
    fn test_multi_property() {
        let schema = Schema::from_text("{id: string, email: string}").unwrap();
        assert!(
            Validator::validate(
                &schema.root,
                r#"{"id": "xxx-yyy", "email": "admin@example.com"}"#
            )
            .unwrap()
        )
    }

    #[test]
    fn test_nested_object() {
        let schema = Schema::from_text("{address: {street: string}}").unwrap();
        assert!(
            Validator::validate(&schema.root, r#"{"address": {"street": "Main St."}}"#).unwrap()
        )
    }

    #[test]
    fn test_property_not_found() {
        let schema = Schema::from_text("{id: string, name: string}") .unwrap();
        assert_eq!(
            Validator::validate(&schema.root, r#"{"name": "John"}"#).unwrap_err(),
            ValidationErrors(vec![ValidationError::PropertyNotFound {
                name: "id".to_string()
            }])
        )
    }
}
