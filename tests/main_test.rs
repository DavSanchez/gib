use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};
use tempfile::tempdir; // Used for writing assertions

#[test]
fn gib_at_cwd() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn gib_at_output_path() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}
#[test]
fn gitignore_exists_at_output_path() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let file_path = dir.path().join(".gitignore");

    let mut file = File::create(&file_path)?;
    writeln!(file, "# Dummy .gitignore")?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("go rust -o").arg(file_path.as_path());
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: .gitignore file already exists at this location.",
    ));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn output_path_is_file() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn gitignore_exists_at_cwd() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented (Can I even test this without messing with local .gitignore?");
}

#[test]
fn unknown_template() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn no_template() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn list_flag() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn show_flag() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}

#[test]
fn show_and_output_flag() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Not implemented");
}
