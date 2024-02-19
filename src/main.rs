use std::io::{stdin, BufReader, BufRead};

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
struct Schema {
    terms: Vec<Term>,
}

impl Schema {
    fn new() -> Schema {
        Schema { terms: vec![] }
    }

    fn add_term(self: &mut Schema, term: Term) {
        self.terms.push(term);
    }

    fn from_text(text: &str) -> Result<Schema, String> {
        let mut schema = Schema::new();

        let terms = text.split_whitespace();
        for term in terms {
            let term = Term::from_text(term)?;
            schema.add_term(term);
        }

        Ok(schema)
    }
}

fn validate(schema: &Schema, line: &str) -> Result<(), String> {
    let values = line.split_whitespace().collect::<Vec<&str>>();

    if values.len() != schema.terms.len() {
        return Err(format!("Invalid number of values: {}", values.len()));
    }

    for (i, value) in values.iter().enumerate() {
        let term = &schema.terms[i];
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

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    let schema = Schema::from_text(args[1].as_str())?;

    // reader from file or stdin
    let reader: Box<dyn BufRead> = match args.get(2) {
        Some(file_name) => Box::new(BufReader::new(std::fs::File::open(file_name).unwrap())),
        None => Box::new(BufReader::new(stdin())),
    };

    for line in reader.lines() {
        let line = line.unwrap();
        validate(&schema, line.as_str())?;
    }

    Ok(())
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
    assert_eq!(validate(&schema, "1 john_doe true\t100.0 _ _").unwrap(), ());
    assert_eq!(validate(&schema, "1 john_doe true\t100.0 _ text").unwrap(), ());

    // multiple whitespaces
    assert_eq!(validate(&schema, "1  john_doe  true  100.0  _ _").unwrap(), ());

    // incorrect values
    assert_eq!(validate(&schema, "1 john_doe true 100.0 _ _ extra").unwrap_err(), "Invalid number of values: 7");
}
