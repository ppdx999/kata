use thiserror::Error;
use std::io::BufRead;
use crate::tsv;
use crate::json;

#[derive(Error, Debug, PartialEq)]
pub enum SchemaError {
    #[error(transparent)]
    Tsv(#[from] tsv::SchemaErrors),

    #[error(transparent)]
    Json(#[from] json::SchemaErrors),
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error(transparent)]
    Tsv(#[from] tsv::ValidationErrors),

    #[error(transparent)]
    Json(#[from] json::ValidationErrors),
}

pub enum Schema {
    Tsv(tsv::Schema),
    Json(json::Schema),
}

impl Schema {
    pub fn from_text(schema_type: &str, text: &str) -> Result<Self, SchemaError> {
        match schema_type {
            "tsv" => Ok(Schema::Tsv(tsv::Schema::from_text(text)?)),
            "json" => Ok(Schema::Json(json::Schema::from_text(text)?)),
            _ => panic!("Unknown schema type: {}", schema_type),
        }
    }
    pub fn print_and_validate(&self, reader: Box<dyn BufRead>) -> Result<(), ValidationError> {
        match self {
            Schema::Tsv(schema) => Ok(schema.print_and_validate(reader)?),
            Schema::Json(schema) => Ok(schema.print_and_validate(reader)?),
        }
    }
}
