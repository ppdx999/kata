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

#[cfg(test)]
mod tests {
    use super::*;

    mod term_from_text {
        use super::*;

        #[test]
        fn primitives() {
            assert_eq!(Term::from_text("id:integer").unwrap(), Term::new("id", vec![Type::Integer]));
            assert_eq!(Term::from_text("name:string").unwrap(), Term::new("name", vec![Type::String]));
            assert_eq!(Term::from_text("is_active:boolean").unwrap(), Term::new("is_active", vec![Type::Boolean]));
            assert_eq!(Term::from_text("price:float").unwrap(), Term::new("price", vec![Type::Float]));
            assert_eq!(Term::from_text("deleted_field:null").unwrap(), Term::new("deleted_field", vec![Type::Null]));
        }

        #[test]
        fn case_insensitive() {
            assert_eq!(Term::from_text("id:INTEGER").unwrap(), Term::new("id", vec![Type::Integer]));
            assert_eq!(Term::from_text("name:STRING").unwrap(), Term::new("name", vec![Type::String]));
            assert_eq!(Term::from_text("is_active:BOOLEAN").unwrap(), Term::new("is_active", vec![Type::Boolean]));
            assert_eq!(Term::from_text("price:FLOAT").unwrap(), Term::new("price", vec![Type::Float]));
            assert_eq!(Term::from_text("deleted_field:NULL").unwrap(), Term::new("deleted_field", vec![Type::Null]));
        }

        #[test]
        fn invalid_type() {
            assert_eq!(Term::from_text("id:unknown").unwrap_err(), SchemaError::InvalidType {
                type_: "unknown".to_string()
            });
        }

        #[test]
        fn invalid_syntax() {
            assert_eq!(Term::from_text("id").unwrap_err(), SchemaError::InvalidSyntax {
                text: "id".to_string()
            });
        }
    }

    mod term_validate {
        use super::*;

        #[test]
        fn integer() {
            assert_eq!(Term::new("id", vec![Type::Integer]).validate("123"), Ok(()));
            assert_eq!(
                Term::new("id", vec![Type::Integer]).validate("123.0").unwrap_err(),
                ValidateLineError::DataTypeMismatch {
                    type_: "integer".to_string(),
                    value: "123.0".to_string()
                }
            );
        }

        #[test]
        fn string() {
            assert_eq!(Term::new("name", vec![Type::String]).validate("John Doe"), Ok(()));
        }

        #[test]
        fn boolean() {
            assert_eq!(Term::new("is_active", vec![Type::Boolean]).validate("true"), Ok(()));
            assert_eq!(Term::new("is_active", vec![Type::Boolean]).validate("false"), Ok(()));
            assert_eq!(
                Term::new("is_active", vec![Type::Boolean]).validate("TURE").unwrap_err(),
                ValidateLineError::DataTypeMismatch {
                    type_: "boolean".to_string(),
                    value: "TURE".to_string()
                }
            );
        }

        #[test]
        fn float() {
            assert_eq!(Term::new("price", vec![Type::Float]).validate("123.0"), Ok(()));
            assert_eq!(Term::new("price", vec![Type::Float]).validate("123"), Ok(()));
            assert_eq!(
                Term::new("price", vec![Type::Float]).validate("123.0.0").unwrap_err(),
                ValidateLineError::DataTypeMismatch {
                    type_: "float".to_string(),
                    value: "123.0.0".to_string()
                }
            );
        }

        #[test]
        fn null() {
            assert_eq!(Term::new("deleted_field", vec![Type::Null]).validate("_"), Ok(()));
        }
    }
}
