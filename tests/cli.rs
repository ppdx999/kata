use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

static CMD_NAME: &'static str = "kata";

#[test]
fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("id:integer");

    // act & assert
    cmd.assert().success().stdout(
        predicate::str::contains("schema: id:integer")
    );
    Ok(())
}

