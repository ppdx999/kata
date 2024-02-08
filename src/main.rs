use std::io::{stdin, BufReader, BufRead};

#[derive(Debug, PartialEq)]
enum Type {
    Integer,
    Float,
    String,
    Boolean,
}

#[derive(Debug, PartialEq)]
struct Term {
    name: String,
    type_: Type,
}

impl Term {
    fn new(name: &str, type_: Type) -> Term {
        Term {
            name: name.to_string(),
            type_,
        }
    }

    fn from_text(text: &str) -> Result<Term, String> {
        let inputs = text.split(":").collect::<Vec<&str>>();
        if inputs.len() != 2 {
            return Err(format!("Invalid term: {}", text));
        }
        let name = inputs[0].to_string();
        let type_ = inputs[1].to_lowercase();

        let type_ = match type_.as_str() {
            "integer" => Type::Integer,
            "float" => Type::Float,
            "string" => Type::String,
            "boolean" => Type::Boolean,
            _ => return Err(format!("Invalid type: {}", type_)),
        };

        Ok(Term::new(name.as_str(), type_))
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
        match term.type_ {
            Type::Integer => {
                value.parse::<i64>().map(|_| ()).map_err(|e| e.to_string())?;
            }
            Type::Float => {
                value.parse::<f64>().map(|_| ()).map_err(|e| e.to_string())?;
            }
            Type::String => {}
            Type::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "false" => {}
                    _ => return Err(format!("Invalid boolean: {}", value)),
                }
            }
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
    assert_eq!(Term::from_text("id:integer").unwrap(), Term::new("id", Type::Integer));
    assert_eq!(Term::from_text("name:string").unwrap(), Term::new("name", Type::String));
    assert_eq!(Term::from_text("is_active:boolean").unwrap(), Term::new("is_active", Type::Boolean));
    assert_eq!(Term::from_text("price:float").unwrap(), Term::new("price", Type::Float));
    assert_eq!(Term::from_text("price:FLOAT").unwrap(), Term::new("price", Type::Float));

    // incorrect schema
    assert_eq!(Term::from_text("id:binary").unwrap_err(), "Invalid type: binary");
    assert_eq!(Term::from_text("id").unwrap_err(), "Invalid term: id");
}


#[test]
fn test_schema_from_text() {
    // correct schema
    assert_eq!(Schema::from_text("id:integer name:string is_active:boolean price:float").unwrap(), Schema {
        terms: vec![
            Term::new("id", Type::Integer),
            Term::new("name", Type::String),
            Term::new("is_active", Type::Boolean),
            Term::new("price", Type::Float),
        ]
    });

    // Accept extra space in field separator
    assert_eq!(Schema::from_text("id:integer \t name:string").unwrap(), Schema {
        terms: vec![
            Term::new("id", Type::Integer),
            Term::new("name", Type::String),
        ]
    });

    // incorrect schema
    assert_eq!(Schema::from_text("id:integer name:string is_active:boolean price:float err:extra").unwrap_err(), "Invalid type: extra");
}


#[test]
fn test_validate() {
    let schema = Schema::from_text("id:integer name:string is_active:boolean price:float").unwrap();

    // correct values
    assert_eq!(validate(&schema, "1 john_doe true\t100.0").unwrap(), ());

    // multiple whitespaces
    assert_eq!(validate(&schema, "1  john_doe  true  100.0").unwrap(), ());

    // incorrect values
    assert_eq!(validate(&schema, "1 john_doe true 100.0 extra").unwrap_err(), "Invalid number of values: 5");
}
