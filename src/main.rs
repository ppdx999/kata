use std::io::{stdin, BufReader, BufRead};

mod cli;
mod tsv;

use clap::Parser;
use cli::Cli;
use tsv::Schema;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let schema = Schema::from_text(cli.schema.as_str())?;

    // reader from file or stdin
    let reader: Box<dyn BufRead> = match cli.file {
        Some(file_name) => Box::new(BufReader::new(std::fs::File::open(file_name).unwrap())),
        None => Box::new(BufReader::new(stdin())),
    };

    for line in reader.lines() {
        let line = line.unwrap();
        schema.validate(line)?;
    }

    Ok(())
}
