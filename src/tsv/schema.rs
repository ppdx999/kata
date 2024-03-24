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
