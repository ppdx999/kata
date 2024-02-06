use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

static CMD_NAME: &'static str = "kata";

#[test]
fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CMD_NAME)?; // Create a new command
    cmd.assert().success().stdout(
        predicate::str::contains("Hello, world!") // Check that the command succeeds and that "Hello, world!" is printed to stdout
    );
    Ok(())
}
