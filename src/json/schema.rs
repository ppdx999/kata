use std::io::BufRead;
use crate::json::data::Node;
use crate::json::parser::Parser;
use crate::json::validator::Validator;

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub root: Node,
}

impl Schema {
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut parser = Parser::new(text);
        Ok(Schema { root: parser.parse() })
    }

    pub fn validate(&self, rdr: Box<dyn BufRead>) -> Result<(), String> {
        let text = rdr.lines().collect::<Result<Vec<String>, _>>().map_err(|e| e.to_string())?.join("\n");
        Validator::validate(self, &text).map(|_| ())
    }
}

#[test]
fn test_schema_from_text() {
    use crate::json::data::{Node, NodeKind};

    let schema = Schema::from_text("{}").unwrap();
    assert_eq!(schema, Schema {
        root: Node::new(NodeKind::Object),
    });
}

#[test]
fn test_schema_validate() {
    let schema = Schema::from_text("{}").unwrap();
    assert!(schema.validate(Box::new("{}".as_bytes())).is_ok());
}

