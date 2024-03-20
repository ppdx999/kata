use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SchemaError {
    #[error(r#"
        Error: Invalid Syntax

        Expect: <id>:<type>
        But, Found: {text}
    "#)]
    InvalidSyntax {
        text: String
    },

    #[error(r#"
        Error: Invalid Type {type_}

        Available types: integer, float, string, boolean, null
    "#)]
    InvalidType {
        type_: String
    }
}

#[derive(Error, Debug, PartialEq)]
pub struct SchemaErrors(pub Vec<SchemaError>);

impl std::fmt::Display for SchemaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.0 {
            write!(f, "{}\n", error)?;
        }
        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidateLineError {
    #[error(r#"
        Error: Data type mismatch

        Expected type is {type_}, But Found: {value}
    "#)]
    DataTypeMismatch {
        type_: String,
        value: String
    },

    #[error(r#"
        Error: Field number mismatch
        Expected: {expected}, But Found: {found}
    "#)]
    FieldNumberMismatch {
        expected: usize,
        found: usize,
    }
}

#[derive(Error, Debug, PartialEq)]
pub struct ValidateLineErrors(pub Vec<ValidateLineError>);

impl std::fmt::Display for ValidateLineErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "----------------\n")?;
        for error in &self.0 {
            write!(f, "{}\n", error)?;
            write!(f, "----------------\n")?;
        }
        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
#[error(r#"
    Error: Validation failed at line {line_number}
    Raw text: {line_text}

    {errors}
"#)]
pub struct ValidationError {
    pub line_text: String,
    pub line_number: usize,
    pub errors: ValidateLineErrors,
}

#[derive(Error, Debug, PartialEq)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.0 {
            write!(f, "{}\n", error)?;
        }
        Ok(())
    }
}
