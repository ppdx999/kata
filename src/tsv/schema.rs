use std::io::BufRead;
use super::term::Term;
use super::error::{
    SchemaErrors,
    ValidationError,
    ValidationErrors,
    ValidateLineError,
    ValidateLineErrors,
};

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

    pub fn from_text(text: &str) -> Result<Schema, SchemaErrors> {
        let mut schema = Schema::new();
        let mut errors = vec![];

        let terms = text.split_whitespace();
        for term in terms {
            match Term::from_text(term) {
                Ok(term) => schema.add_term(term),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(schema)
        } else {
            Err(SchemaErrors(errors))
        }
    }

    fn validate_line(self: &Schema, line: String) -> Result<(), ValidateLineErrors> {
        let values = line.split_whitespace().collect::<Vec<&str>>();
        if values.len() != self.terms.len() {
            return Err(ValidateLineErrors(
                vec![
                    ValidateLineError::FieldNumberMismatch {
                        expected: self.terms.len(),
                        found: values.len()
                    }
                ]
            ));
        }

        let mut errors = vec![];

        for (i, value) in values.iter().enumerate() {
            let term = &self.terms[i];
            match term.validate(value) {
                Ok(_) => (),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidateLineErrors(errors))
        }
    }

    pub fn print_and_validate(self: &Schema, reader: Box<dyn BufRead>) -> Result<(), ValidationErrors> {
        let mut errors = vec![];

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            println!("{}", line);

            match self.validate_line(line.clone()) {
                Ok(_) => (),
                Err(line_errors) => errors.push(
                    ValidationError {
                        line_number: i + 1,
                        line_text: line,
                        errors: line_errors,
                    }
                )
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tsv::term::Type;
    use crate::tsv::error::{SchemaError, ValidateLineError};
    
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
            assert_eq!(
                Schema::from_text("id:integer err:extra").unwrap_err(),
                SchemaErrors(vec![
                    SchemaError::InvalidType{
                        type_: "extra".to_string()
                    }
                ])
            )
        }

        #[test]
        fn incorrect_syntax() {
            assert_eq!(
                Schema::from_text("id").unwrap_err(),
                SchemaErrors(vec![
                    SchemaError::InvalidSyntax {
                        text: "id".to_string()
                    }
                ])
            )
        }
    }

    mod validate_line {
        use super::*;

        #[test]
        fn integer() {
            let schema = Schema::from_text("id:integer").unwrap();
            assert_eq!(schema.validate_line("123".to_owned()), Ok(()));

            assert_eq!(
                schema.validate_line("123.0".to_owned()),
                Err(
                    ValidateLineErrors(
                        vec![
                            ValidateLineError::DataTypeMismatch {
                                type_: "integer".to_string(),
                                value: "123.0".to_string()
                            }
                        ]
                    )
                )
            );
        }

        #[test]
        fn string() {
            let schema = Schema::from_text("name:string").unwrap();

            assert_eq!(schema.validate_line("John_Doe".to_owned()), Ok(()));
        }

        #[test]
        fn boolean() {
            let schema = Schema::from_text("is_active:boolean").unwrap();

            assert_eq!(schema.validate_line("true".to_owned()), Ok(()));
            assert_eq!(schema.validate_line("false".to_owned()), Ok(()));
            assert_eq!(
                schema.validate_line("TURE".to_owned()),
                Err(
                    ValidateLineErrors(
                        vec![
                            ValidateLineError::DataTypeMismatch {
                                type_: "boolean".to_string(),
                                value: "TURE".to_string()

                            }
                        ]
                    )
                )
            );
        }

        #[test]
        fn float() {
            let schema = Schema::from_text("price:float").unwrap();
            assert_eq!(schema.validate_line("123.0".to_owned()), Ok(()));
            assert_eq!(schema.validate_line("123".to_owned()), Ok(()));
            assert_eq!(
                schema.validate_line("123.0.0".to_owned()),
                Err(
                    ValidateLineErrors(
                        vec![
                            ValidateLineError::DataTypeMismatch {
                                type_: "float".to_string(),
                                value: "123.0.0".to_string()
                            }
                        ]
                    )
                )
            )
        }

        #[test]
        fn null() {
            let schema = Schema::from_text("deleted_field:null").unwrap();
            assert_eq!(schema.validate_line("_".to_owned()), Ok(()));
        }

        #[test]
        fn multiple_terms() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("1 john_doe".to_owned()), Ok(()));
        }

        #[test]
        fn accept_extra_space() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(schema.validate_line("1       john_doe".to_owned()), Ok(()));
        }

        #[test]
        fn incorrect_number_of_values() {
            let schema = Schema::from_text("id:integer name:string").unwrap();
            assert_eq!(
                schema.validate_line("1".to_owned()),
                Err(
                    ValidateLineErrors(
                        vec![
                            ValidateLineError::FieldNumberMismatch {
                                expected: 2,
                                found: 1
                            }
                        ]
                    )
                )
            )
        }

        #[test]
        fn incorrect_value() {
            let schema = Schema::from_text("id:integer name:integer").unwrap();
            assert_eq!(
                schema.validate_line("xxx yyy".to_owned()),
                Err(
                    ValidateLineErrors(
                        vec![
                            ValidateLineError::DataTypeMismatch {
                                type_: "integer".to_string(),
                                value: "xxx".to_string()
                            },
                            ValidateLineError::DataTypeMismatch {
                                type_: "integer".to_string(),
                                value: "yyy".to_string()
                            }
                        ]
                    )
                )
            )
        }
    }
}
