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
            schema::Type::Null => match value {
                Value::Null => Ok(true),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "null".to_string(),
                    value: value.to_string(),
                }.into()),
            },
            schema::Type::String => match value {
                Value::String(_) => Ok(true),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "string".to_string(),
                    value: value.to_string(),
                }.into()),
            },
            schema::Type::Number => match value {
                Value::Number(_) => Ok(true),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "number".to_string(),
                    value: value.to_string()
                }.into())
            },
            schema::Type::Boolean => match value {
                Value::Bool(_) => Ok(true),
                _ => Err(ValidationError::DataTypeMismatch {
                    type_: "boolean".to_string(),
                    value: value.to_string()
                }.into())
            }
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
