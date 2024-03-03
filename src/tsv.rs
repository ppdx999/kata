#[derive(Debug, PartialEq)]
enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Null,
}

#[derive(Debug, PartialEq)]
struct Term {
    name: String,
    types: Vec<Type>,
}

impl Term {
    fn new(name: &str, types: Vec<Type>) -> Term {
        Term {
            name: name.to_string(),
            types,
        }
    }

    fn from_text(text: &str) -> Result<Term, String> {
        let inputs = text.split(":").collect::<Vec<&str>>();
        if inputs.len() != 2 {
            return Err(format!("Invalid term: {}", text));
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
                _ => return Err(format!("Invalid type: {}", type_)),
            }
        }).collect::<Result<Vec<Type>, String>>()?;

        Ok(Term::new(name.as_str(), types))
    }
}

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

    pub fn validate(self: &Schema, line: String) -> Result<(), String> {
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
}

#[test]
fn test_term_from_text() {
    // correct schema
    assert_eq!(Term::from_text("id:integer").unwrap(), Term::new("id", vec![Type::Integer]));
    assert_eq!(Term::from_text("name:string").unwrap(), Term::new("name", vec![Type::String]));
    assert_eq!(Term::from_text("is_active:boolean").unwrap(), Term::new("is_active", vec![Type::Boolean]));
    assert_eq!(Term::from_text("price:float").unwrap(), Term::new("price", vec![Type::Float]));
    assert_eq!(Term::from_text("price:FLOAT").unwrap(), Term::new("price", vec![Type::Float]));
    assert_eq!(Term::from_text("deleted_field:null").unwrap(), Term::new("deleted_field", vec![Type::Null]));

    // incorrect schema
    assert_eq!(Term::from_text("id:binary").unwrap_err(), "Invalid type: binary");
    assert_eq!(Term::from_text("id").unwrap_err(), "Invalid term: id");
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
fn test_validate() {
    let schema = Schema::from_text("id:integer name:string is_active:boolean price:float deleted_field:null optional_field:string|null").unwrap();

    // correct values
    assert_eq!(schema.validate("1 john_doe true 100.0 _ _".to_owned()).unwrap(), ());
    assert_eq!(schema.validate("1 john_doe true\t100.0 _ _".to_owned()).unwrap(), ());

    // // multiple whitespaces
    assert_eq!(schema.validate("1  john_doe  true  100.0  _ _".to_owned()).unwrap(), ());

    // // incorrect values
    assert_eq!(schema.validate("1 john_doe true 100.0 _ _ extra".to_owned()).unwrap_err(), "Invalid number of values: 7");
}
