use std::io::BufRead;
use crate::tsv::term::{Term, Type};

#[derive(Debug, PartialEq)]
pub struct Schema {
    terms: Vec<Term>,
}

impl Schema {
    fn new() -> Schema {
        Schema { terms: vec![] }
    }

    fn add_term(self: &mut Schema, term: Term) {
        self.terms.push(term);
    }

    pub fn from_text(text: &str) -> Result<Schema, String> {
        let mut schema = Schema::new();

        let terms = text.split_whitespace();
        for term in terms {
            let term = Term::from_text(term)?;
            schema.add_term(term);
        }

        Ok(schema)
    }

    fn validate_line(self: &Schema, line: String) -> Result<(), String> {
        let values = line.split_whitespace().collect::<Vec<&str>>();
        if values.len() != self.terms.len() {
            return Err(format!("Invalid number of values: {}", values.len()));
        }
        for (i, value) in values.iter().enumerate() {
            let term = &self.terms[i];
            let mut is_valid = false;
            for type_ in &term.types {
                match type_ {
                    Type::Integer => {
                        if value.parse::<i64>().is_ok() {
                            is_valid = true;
                            break;
                        }
                    }
                    Type::Float => {
                        if value.parse::<f64>().is_ok() {
                            is_valid = true;
                            break;
                        }
                    }
                    Type::String => {
                        is_valid = true;
                        break;
                    }
                    Type::Boolean => {
                        match value.to_lowercase().as_str() {
                            "true" | "false" => {
                                is_valid = true;
                                break;
                            }
                            _ => {}
                        }
                    }
                    Type::Null => {
                        if *value == "_" {
                            is_valid = true;
                            break;
                        }
                    }
                }
            }
            if !is_valid {
                return Err(format!("Invalid value: {}", value));
            }
        }
        Ok(())
    }

    pub fn validate(self: &Schema, reader: Box<dyn BufRead>) -> Result<(), String> {
        for line in reader.lines() {
            let line = line.unwrap();
            self.validate_line(line)?;
        }
        Ok(())
    }
}

#[test]
fn test_schema_from_text() {
    // correct schema
    assert_eq!(Schema::from_text("id:integer name:string is_active:boolean price:float deleted_field:null optional_field:string|null").unwrap(), Schema {
        terms: vec![
            Term::new("id", vec![Type::Integer]),
            Term::new("name", vec![Type::String]),
            Term::new("is_active", vec![Type::Boolean]),
            Term::new("price", vec![Type::Float]),
            Term::new("deleted_field", vec![Type::Null]),
            Term::new("optional_field", vec![Type::String, Type::Null]),
        ]
    });

    // Accept extra space in field separator
    assert_eq!(Schema::from_text("id:integer \t name:string").unwrap(), Schema {
        terms: vec![
            Term::new("id", vec![Type::Integer]),
            Term::new("name", vec![Type::String]),
        ]
    });

    // incorrect schema
    assert_eq!(Schema::from_text("id:integer name:string is_active:boolean price:float err:extra").unwrap_err(), "Invalid type: extra");
}

#[test]
fn test_validate_line() {
    let schema = Schema::from_text("id:integer name:string is_active:boolean price:float deleted_field:null optional_field:string|null").unwrap();

    // correct values
    assert_eq!(schema.validate_line("1 john_doe true 100.0 _ _".to_owned()).unwrap(), ());
    assert_eq!(schema.validate_line("1 john_doe true\t100.0 _ _".to_owned()).unwrap(), ());

    // // multiple whitespaces
    assert_eq!(schema.validate_line("1  john_doe  true  100.0  _ _".to_owned()).unwrap(), ());

    // // incorrect values
    assert_eq!(schema.validate_line("1 john_doe true 100.0 _ _ extra".to_owned()).unwrap_err(), "Invalid number of values: 7");
}
