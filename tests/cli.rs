use assert_cmd::prelude::*;
// use predicates::prelude::*;
use std::process::Command;

static CMD_NAME: &'static str = "kata";

#[test]
fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("id:integer").arg("123");

    // act & assert
    cmd.assert().success();
    Ok(())
}
