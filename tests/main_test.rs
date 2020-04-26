use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};
use tempfile::tempdir; // Used for writing assertions

#[test]
fn gitignore_path_exists() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let file_path = dir.path().join(".gitignore");

    let mut file = File::create(&file_path)?;
    writeln!(file, "# Dummy .gitignore")?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("go rust -o").arg(file_path.as_path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: .gitignore file already exists at this location."));

    drop(file);
    dir.close()?;

    Ok(())
}
