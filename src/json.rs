mod data;
mod lexer;
mod parser;
mod schema;
mod validator;
mod error;

pub use schema::Schema;
pub use error::{SchemaErrors, ValidationErrors};
