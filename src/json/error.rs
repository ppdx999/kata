use thiserror::Error;
use serde_json::Error as SerdeError;
use super::data::{TokenKind, Location };

#[derive(Error, Debug, PartialEq)]
pub enum SchemaError {
    #[error("Error: Invalid Syntax, Unexpected character {text} found at {location}")]
    UnexpectedCharacter {
        text: String,
        location: Location,
    },

    #[error(r#"
        Error: Invalid Syntax, Unexpected token found at {location}

        Expected token: {expected_kind:?}
        But Found: {actual_kind:?}
    "#)]
    UnexpectedToken {
        expected_kind: TokenKind,
        location: Location,
        actual_kind: TokenKind,
    },

    #[error(r#"
        Error: Invalid Type {type_} found at {location}

        Available types: integer, float, string, boolean, null
    "#)]
    InvalidType {
        type_: String,
        location: Location,
    },

    #[error("Error: Unterminated string found at {location}")]
    UnterminatedString {
        location: Location,
    },
}

#[derive(Error, Debug, PartialEq)]
pub struct SchemaErrors(pub Vec<SchemaError>);

impl From<SchemaError> for SchemaErrors {
    fn from(error: SchemaError) -> SchemaErrors {
        SchemaErrors(vec![error])
    }
}

impl std::fmt::Display for SchemaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.0 {
            write!(f, "{}\n", error)?;
        }
        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("{0}")]
    ParseFaild(String),

    #[error("Error: Property not found: {name}")]
    PropertyNotFound {
        name: String,
    },

    #[error(r#"
        Error: Data type mismatch

        Expected type is {types}, But Found: {value}
    "#)]
    DataTypeMismatch {
        types: String,
        value: String
    },
}

impl From<SerdeError> for ValidationError {
    fn from(error: SerdeError) -> ValidationError {
        ValidationError::ParseFaild(error.to_string())
    }
}

#[derive(Error, Debug, PartialEq)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl ValidationErrors {
    pub fn new() -> ValidationErrors {
        ValidationErrors(vec![])
    }

    pub fn extend(self: &mut ValidationErrors, other: ValidationErrors) {
        self.0.extend(other.0);
    }

    pub fn is_empty(self: &ValidationErrors) -> bool {
        self.0.is_empty()
    }
}

impl From<ValidationError> for ValidationErrors {
    fn from(error: ValidationError) -> ValidationErrors {
        ValidationErrors(vec![error])
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.0 {
            write!(f, "{}\n", error)?;
        }
        Ok(())
    }
}
