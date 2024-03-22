use std::io::{stdin, BufReader, BufRead};

mod schema;
mod cli;
mod tsv;
mod json;
mod error;
mod result;

use clap::Parser;
use cli::Cli;
use schema::Schema;
use result::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let schema = Schema::from_text(cli.schema_type.to_string().as_str(), cli.schema.as_str())?;

    // reader from file or stdin
    let reader: Box<dyn BufRead> = match cli.file {
        Some(file_name) => Box::new(BufReader::new(std::fs::File::open(file_name).unwrap())),
        None => Box::new(BufReader::new(stdin())),
    };

    if let Err(e) = schema.validate(reader) {
        eprintln!("{}", e);
        std::process::exit(1);
    };

    Ok(())
}
