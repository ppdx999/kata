use assert_cmd::Command;
// use assert_cmd::prelude::*;
// use predicates::prelude::*;
// use std::process::Command;

static CMD_NAME: &'static str = "kata";

#[test]
fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("id:integer email:string is_active:boolean")
        .write_stdin("1 jhon_doe@example.com true\n2 emily_lua@example.com false\n3 mac_kily@example.com true");

    // act & assert
    cmd.assert().success();
    Ok(())
}
