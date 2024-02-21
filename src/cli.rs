use clap::Parser;

#[derive(Parser)]
#[command(name = "schematch", version, author, about = "Schema checking tool")]
pub struct Cli {
    /// The schema to check against.
    /// ex. "id:integer email:string name:string"
    pub schema: String,
    /// The file to check. If not provided, stdin will be used.
    pub file: Option<String>,
}
