use std::io::BufRead;
use crate::tsv::term::Term;

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
            // TODO: should combined all error messages and return
            term.validate(value)?;
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

#[cfg(test)]
mod tests {
    use crate::tsv::term::{Term, Type};
    use crate::tsv::schema::Schema;
    
    mod schema_from_text {
        use super::*;

        #[test]
        fn single_term() {
            assert_eq!(Schema::from_text("id:integer").unwrap(), Schema {
                terms: vec![
                    Term::new("id", vec![Type::Integer])
                ]
            });

            assert_eq!(Schema::from_text("name:string").unwrap(), Schema {
                terms: vec![
                    Term::new("name", vec![Type::String])
                ]
            });
            
            assert_eq!(Schema::from_text("is_active:boolean").unwrap(), Schema {
                terms: vec![
                    Term::new("is_active", vec![Type::Boolean])
                ]
            });

            assert_eq!(Schema::from_text("price:float").unwrap(), Schema {
                terms: vec![
                    Term::new("price", vec![Type::Float])
                ]
            });

            assert_eq!(Schema::from_text("deleted_field:null").unwrap(), Schema {
                terms: vec![
                    Term::new("deleted_field", vec![Type::Null])
                ]
            });
        }

        #[test]
        fn sum_term() {
            assert_eq!(Schema::from_text("optional_field:string|null").unwrap(), Schema {
                terms: vec![
                    Term::new("optional_field", vec![Type::String, Type::Null])
                ]
            });
        }

        #[test]
        fn multiple_terms() {
            assert_eq!(Schema::from_text("id:integer name:string").unwrap(), Schema {
            terms: vec![
                Term::new("id", vec![Type::Integer]),
                Term::new("name", vec![Type::String])
            ]
            });
        }

        #[test]
        fn accept_extra_space() {
            assert_eq!(Schema::from_text("id:integer \t name:string").unwrap(), Schema {
            terms: vec![
                Term::new("id", vec![Type::Integer]),
                Term::new("name", vec![Type::String])
            ]
            });
        }

        #[test]
        fn incorrect_schema() {
            assert_eq!(Schema::from_text("id:integer err:extra").unwrap_err(), "Invalid type: extra");
        }
    }

    mod validate_line {
        use super::*;

        #[test]
        fn integer() {
            let schema = Schema::from_text("id:integer").unwrap();
            assert_eq!(schema.validate_line("123".to_owned()).unwrap(), ());
            assert_eq!(schema.validate_line("123.0".to_owned()).unwrap_err(), "Invalid value: 123.0");
        }

        #[test]
        fn string() {
            let schema = Schema::from_text("name:string").unwrap();
            assert_eq!(schema.validate_line("John_Doe".to_owned()).unwrap(), ());
        }

        #[test]
        fn boolean() {
            let schema = Schema::from_text("is_active:boolean").unwrap();
            assert_eq!(schema.validate_line("true".to_owned()).unwrap(), ());
            assert_eq!(schema.validate_line("false".to_owned()).unwrap(), ());
            assert_eq!(schema.validate_line("TURE".to_owned()).unwrap_err(), "Invalid value: TURE");
        }

        #[test]
        fn float() {
            let schema = Schema::from_text("price:float").unwrap();
            assert_eq!(schema.validate_line("123.0".to_owned()).unwrap(), ());
            assert_eq!(schema.validate_line("123".to_owned()).unwrap(), ());
            assert_eq!(schema.validate_line("123.0.0".to_owned()).unwrap_err(), "Invalid value: 123.0.0");
        }

        #[test]
        fn null() {
            let schema = Schema::from_text("deleted_field:null").unwrap();
            assert_eq!(schema.validate_line("_".to_owned()).unwrap(), ());
        }

        #[test]
        fn multiple_terms() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("1 john_doe".to_owned()).unwrap(), ());
        }

        #[test]
        fn accept_extra_space() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("1       john_doe".to_owned()).unwrap(), ());
        }

        #[test]
        fn incorrect_number_of_values() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("1".to_owned()).unwrap_err(), "Invalid number of values: 1");
        }

        #[test]
        fn incorrect_value() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("xxx yyy".to_owned()).unwrap_err(), "Invalid value: xxx");
        }
    }
}
