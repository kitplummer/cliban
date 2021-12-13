use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_default_configuration_write() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cliban")?;
    cmd.arg("configure");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("default configuration file"));
    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cliban")?;
    cmd.arg("help");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("A command-line kanban board"));
    Ok(())
}