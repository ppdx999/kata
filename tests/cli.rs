use assert_cmd::Command;
// use assert_cmd::prelude::*;
// use predicates::prelude::*;
// use std::process::Command;

static CMD_NAME: &'static str = "schematch";

#[test]
fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("id:integer email:string is_active:boolean name:string|null")
        .write_stdin("1 jhon_doe@example.com true Jhon_Doe\n2 emily_lua@example.com false Emily_Lua\n3 mac_kily@example.com true _");

    // act & assert
    cmd.assert().success();
    Ok(())
}

#[test]
fn run_program_with_options() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("id:integer email:string is_active:boolean name:string|null")
        .arg("--schema-type")
        .arg("tsv")
        .write_stdin("1 jhon_doe@example.com true Jhon_Doe\n2 emily_lua@example.com false Emily_Lua\n3 mac_kily@example.com true _");

    // act & assert
    cmd.assert().success();
    Ok(())
}
