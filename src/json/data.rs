#[derive(Debug, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start: {}, end: {}", self.start, self.end)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Colon,
    Comma,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    VerticalBar,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    pub next: Option<Box<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Null,
    String,
    Number,
    Boolean,
    Object(Box<Object>),
    Array(Box<Array>),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Null => "null".to_string(),
            Type::String => "string".to_string(),
            Type::Number => "number".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Object(_) => "object".to_string(),
            Type::Array(_) => "array".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub types: Vec<Type>,
}

impl Property {
    pub fn new(name: String, types: Vec<Type>) -> Property {
        Property {
            name,
            types,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub properties: Vec<Property>,
}

impl Object {
    pub fn new() -> Object {
        Object { properties: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct Array {
    pub types: Vec<Type>,
}

impl Array {
    pub fn new(types: Vec<Type>) -> Array {
        Array { types }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(Object),
    Array(Array),
    Types(Vec<Type>),
}
