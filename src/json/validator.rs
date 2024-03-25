use serde_json;
use serde_json::{Map, Value};

use super::data as schema;
use super::error::{ValidationError, ValidationErrors};

#[allow(dead_code)]
pub struct Validator;

impl Validator {
    pub fn validate(schema: &schema::Value, text: &str) -> Result<bool, ValidationErrors> {
        let value: serde_json::Value = Self::parse_json(text)?;
        Self::value(schema, &value)
    }

    fn parse_json(text: &str) -> Result<Value, ValidationError> {
        Ok(serde_json::from_str(text)?)
    }

    fn value(schema: &schema::Value, value: &Value) -> Result<bool, ValidationErrors> {
        match schema {
            schema::Value::Object(schema) => match value {
                Value::Object(object) => Self::object(schema, object),
                _ => Err(
                    ValidationError::DataTypeMismatch {
                        type_: "object".to_string(),
                        value: value.to_string(),
                    }.into()
                ),
            },
            schema::Value::Array(schema) => match value {
                Value::Array(array) => Self::array(schema, array),
                _ => Err(
                    ValidationError::DataTypeMismatch {
                        type_: "array".to_string(),
                        value: value.to_string(),
                    }.into()
                ),
            },
            schema::Value::Type(schema) => Self::type_(&schema, value),
        }
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

    fn array(schema: &schema::Array, array: &Vec<Value>) -> Result<bool, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for value in array {
            match Self::type_(&schema.type_, value) {
            Ok(_) => {},
            Err(errs) => errors.extend(errs),
            }
        }

        if errors.is_empty() {
            Ok(true)
        } else {
            Err(errors)
        }
    }

    fn property(schema: &schema::Property, value: &Value) -> Result<bool, ValidationErrors> {
        Self::type_(&schema.type_, value)
    }

    fn type_ (schema: &schema::Type, value: &Value) -> Result<bool, ValidationErrors> {
        match schema {
            schema::Type::Null => match value {
                Value::Null => Ok(true),
                _ => Err(Self::type_mismatch("null", value).into())
            },
            schema::Type::String => match value {
                Value::String(_) => Ok(true),
                _ => Err(Self::type_mismatch("string", value).into())
            },
            schema::Type::Number => match value {
                Value::Number(_) => Ok(true),
                _ => Err(Self::type_mismatch("number", value).into())
            },
            schema::Type::Boolean => match value {
                Value::Bool(_) => Ok(true),
                _ => Err(Self::type_mismatch("boolean", value).into())
            },
            schema::Type::Object(schema) => match value {
                Value::Object(object) => Ok(Self::object(schema, object)?),
                _ => Err(Self::type_mismatch("object", value).into())
            },
            schema::Type::Array(schema) => match value {
                Value::Array(array) => Ok(Self::array(schema, array)?),
                _ => Err(Self::type_mismatch("array", value).into())
            },
        }
    }

    fn type_mismatch(type_: &str, value: &Value) -> ValidationError {
        ValidationError::DataTypeMismatch {
            type_: type_.to_string(),
            value: value.to_string(),
        }
    }
}
