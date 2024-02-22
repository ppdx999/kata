use std::fmt;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum SchemaType {
    Tsv,
    Json,
}

impl fmt::Display for SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaType::Tsv => write!(f, "tsv"),
            SchemaType::Json => write!(f, "json"),
        }
    }
}

#[derive(Parser)]
#[command(name = "schematch", version, author, about = "Declarative schema checking commands")]
pub struct Cli {
    /// The schema to check against.
    /// ex. "id:integer email:string name:string"
    pub schema: String,
    /// The file to check. If not provided, stdin will be used.
    pub file: Option<String>,

    #[clap(short, long)]
    #[arg(default_value_t = SchemaType::Tsv)]
    /// Schema type. schematch support tsv and json, If not provided tsv will be used.
    pub schema_type: SchemaType,
}
