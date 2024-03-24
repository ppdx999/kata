use super::error::{SchemaError, ValidateLineError};

#[derive(Debug, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Null,
}

#[derive(Debug, PartialEq)]
pub struct Term {
    pub name: String,
    pub types: Vec<Type>,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Integer => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Boolean => write!(f, "boolean"),
            Type::Null => write!(f, "null"),
        }
    }
}

impl Term {
    pub fn new(name: &str, types: Vec<Type>) -> Term {
        Term {
            name: name.to_string(),
            types,
        }
    }

    pub fn from_text(text: &str) -> Result<Term, SchemaError> {
        let inputs = text.split(":").collect::<Vec<&str>>();
        if inputs.len() != 2 {
            return Err(SchemaError::InvalidSyntax{
                text: text.to_string()
            });
        }
        let name = inputs[0].to_string();
        let types = inputs[1].to_lowercase();

        let types = types.split("|").map(|type_| {
            match type_ {
                "integer" => Ok(Type::Integer),
                "float" => Ok(Type::Float),
                "string" => Ok(Type::String),
                "boolean" => Ok(Type::Boolean),
                "null" => Ok(Type::Null),
                _ => Err(SchemaError::InvalidType {
                    type_: type_.to_string()
                })
            }
        }).collect::<Result<Vec<Type>, SchemaError>>()?;

        Ok(Term::new(name.as_str(), types))
    }

    pub fn validate(&self, value: &str) -> Result<(), ValidateLineError> {
        let mut is_valid = false;
        for type_ in &self.types {
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
                    if value == "_" {
                        is_valid = true;
                        break;
                    }
                }
            }
        }
        if !is_valid {
            return Err(
                ValidateLineError::DataTypeMismatch {
                    type_: self.types.iter().map(|type_| type_.to_string()).collect::<Vec<String>>().join(" | "),
                    value: value.to_string()
                }
            );
        }
        Ok(())
    }
}
