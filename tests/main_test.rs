use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Add methods on files
use itertools::Itertools;
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};

const GITIGNORE_FILES: &[(&str, (&str, &[u8]))] =
    &include!(concat!(env!("OUT_DIR"), "/gitignore_data.rs"));

#[test]
fn gib_at_cwd() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.current_dir(dir.path()).arg("rust");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created .gitignore file."));

    dir.child(".gitignore").assert(predicates::path::exists());
    // TODO: CHECK FILE CONTENTS ARE THE EXPECTED, HEADERS INCLUDED
    // TODO: CREATE EXAMPLE GITIGNORE OUTPUTS TO COMPARE

    dir.close()?;
    Ok(())
}

#[test]
fn gib_at_output_path() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("rust").arg("-o").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created .gitignore file."));

    dir.child(".gitignore").assert(predicates::path::exists());
    // TODO: CHECK FILE CONTENTS ARE THE EXPECTED, HEADERS INCLUDED
    // TODO: CREATE EXAMPLE GITIGNORE OUTPUTS TO COMPARE
    
    dir.close()?;
    Ok(())
}
#[test]
fn gitignore_exists_at_output_path() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;
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
    let file = assert_fs::NamedTempFile::new("dummy.file")?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("rust").arg("-o").arg(file.path());
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: Output directory does not exist.",
    ));

    Ok(())
}

#[test]
fn gitignore_exists_at_cwd() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;
    let file_path = dir.path().join(".gitignore");
    let mut file = File::create(&file_path)?;
    writeln!(file, "# Dummy .gitignore")?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.current_dir(dir.path()).arg("rust");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: .gitignore file already exists at this location.",
    ));

    dir.close()?;
    Ok(())
}

#[test]
fn unknown_template() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("-s")
        .arg("unknown_template")
        .arg("-o")
        .arg(dir.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Unrecognized template"));

    dir.close()?;
    Ok(())
}

#[test]
fn no_template() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("-s").arg("-o").arg(dir.path());
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: No template arguments provided",
    ));

    dir.close()?;
    Ok(())
}

#[test]
fn list_flag() -> Result<(), Box<dyn std::error::Error>> {
    let templates: Vec<&str> = GITIGNORE_FILES.into_iter().map(|x| x.0).sorted().collect();
    let mut result: String = "".to_string();
    for template in templates {
        result.push_str(&format!("{}\n", template));
    }

    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("-l");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{}", result)));

    Ok(())
}

#[test]
fn show_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("-s").arg("rust");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../gitignore/Rust.gitignore"
        )));

    Ok(())
}

#[test]
fn show_and_output_flag() -> Result<(), Box<dyn std::error::Error>> {
    // Should ignore the -o flag and behave as if using -s alone
    let mut cmd = Command::cargo_bin("gib")?;
    cmd.arg("-s").arg("rust").arg("-o ..");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../gitignore/Rust.gitignore"
        )));

    Ok(())
}
