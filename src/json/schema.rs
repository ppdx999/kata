use std::io::BufRead;

use super::data::Value;
use super::error::{ SchemaErrors, ValidationErrors };
use super::parser::Parser;
use super::validator::Validator;

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub root: Value,
}

impl Schema {
    pub fn from_text(text: &str) -> Result<Self, SchemaErrors> {
        let mut parser = Parser::new(text)?;
        Ok(Schema { root: parser.parse()? })
    }

    pub fn validate(&self, rdr: Box<dyn BufRead>) -> Result<(), ValidationErrors> {
        let text = rdr.lines().collect::<Result<Vec<String>, _>>().unwrap().join("\n");

        Validator::validate(&self.root, &text)?;

        Ok(())
    }
}
