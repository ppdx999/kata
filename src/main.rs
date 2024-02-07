#[derive(Debug, PartialEq)]
enum Type {
    Integer,
    Float,
    String,
    Boolean,
}


fn parse_schema(schema: &str) -> Result<Type, String> {
    let inputs = schema.split(":").collect::<Vec<&str>>();
    let type_name = inputs[1].to_lowercase();

    match type_name.as_str() {
        "integer" => Ok(Type::Integer),
        "float" => Ok(Type::Float),
        "string" => Ok(Type::String),
        "boolean" => Ok(Type::Boolean),
        _ => Err(format!("Invalid type: {}", type_name)),
    }
}

fn validate(t: &Type, value: &str) -> Result<(), String> {
    match t {
        Type::Integer => {
            value.parse::<i64>().map(|_| ()).map_err(|e| e.to_string())
        }
        Type::Float => {
            value.parse::<f64>().map(|_| ()).map_err(|e| e.to_string())
        }
        Type::String => Ok(()),
        Type::Boolean => {
            match value.to_lowercase().as_str() {
                "true" | "false" => Ok(()),
                _ => Err(format!("Invalid boolean: {}", value)),
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    let schema = args[1].as_str();
    let inputs = args[2..].to_vec();

    let primitive_type = parse_schema(&schema)?;
    validate(&primitive_type, inputs[0].as_str())
}


#[test]
fn test_parse_schema() {
    // correct schema
    assert_eq!(parse_schema("id:integer").unwrap(), Type::Integer);
    assert_eq!(parse_schema("name:string").unwrap(), Type::String);
    assert_eq!(parse_schema("is_active:boolean").unwrap(), Type::Boolean );
    assert_eq!(parse_schema("price:float").unwrap(), Type::Float);
    assert_eq!(parse_schema("price:FLOAT").unwrap(), Type::Float);

    // incorrect schema
    assert_eq!(parse_schema("id:binary").unwrap_err(), "Invalid type: binary");
}
