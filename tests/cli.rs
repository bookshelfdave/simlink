extern crate assert_cmd;
extern crate predicates;
extern crate tempfile;

use assert_cmd::prelude::*;
use std::process::Command; // Run programs // Add methods on commands

use std::fs;

use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_a() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    //let mut file = NamedTempFile::new()?;
    let file_path_a = dir.path().join("foo");
    let mut file = File::create(&file_path_a)?;
    writeln!(file, "This is a test file")?;

    let file_path_b = dir.path().join("bar");

    let mut cmd = Command::cargo_bin("simlink")?;
    cmd.arg(&file_path_a);
    cmd.arg(&file_path_b);
    cmd.assert().success();
    let _ = fs::read_to_string(file_path_b).expect("This is a test file");
    Ok(())
}

#[test]
fn test_b() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    //let mut file = NamedTempFile::new()?;
    let file_path_a = dir.path().join("foo");
    let mut file = File::create(&file_path_a)?;
    writeln!(file, "This is a test file")?;

    let file_path_b = dir.path().join("bar");

    let mut cmd = Command::cargo_bin("simlink")?;
    cmd.arg(&file_path_b);
    cmd.arg(&file_path_a);
    cmd.assert().success();
    let _ = fs::read_to_string(file_path_a).expect("This is a test file");
    Ok(())
}
