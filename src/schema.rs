use std::io::BufRead;
use crate::tsv;
use crate::json;

pub enum Schema {
    Tsv(tsv::Schema),
    Json(json::Schema),
}

impl Schema {
    pub fn from_text(schema_type: &str, text: &str) -> Result<Self, String> {
        match schema_type {
            "tsv" => Ok(Schema::Tsv(tsv::Schema::from_text(text)?)),
            "json" => Ok(Schema::Json(json::Schema::from_text(text)?)),
            _ => Err(format!("Unknown schema type: {}", schema_type)),
        }
    }
    pub fn validate(&self, reader: Box<dyn BufRead>) -> Result<(), String> {
        match self {
            Schema::Tsv(schema) => schema.validate(reader),
            Schema::Json(schema) => schema.validate(reader),
        }
    }
}
