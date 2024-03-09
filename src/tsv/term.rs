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

impl Term {
    pub fn new(name: &str, types: Vec<Type>) -> Term {
        Term {
            name: name.to_string(),
            types,
        }
    }

    pub fn from_text(text: &str) -> Result<Term, String> {
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
                _ => Err(format!("Invalid type: {}", type_)),
            }
        }).collect::<Result<Vec<Type>, String>>()?;

        Ok(Term::new(name.as_str(), types))
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
