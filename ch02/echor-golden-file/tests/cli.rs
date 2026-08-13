use anyhow::Result;
use assert_cmd::Command;
use goldenfile::Mint;
use predicates::prelude::*;
use std::io::Write;

fn run(filename: &str, args: &[&str]) {
    let mut mint = Mint::new("tests/testdata");
    let mut file = mint.new_goldenfile(filename).unwrap();
    let output = Command::cargo_bin("echor-golden-file")
        .unwrap()
        .args(args)
        .output()
        .expect("fail");
    let output_string = String::from_utf8(output.stdout).expect("invalid UTF-8");
    write!(file, "{}", output_string).unwrap();
}

#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin("echor-golden-file")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() {
    run("hello1.txt", &["Hello there"]);
}

#[test]
fn hello2() {
    run("hello2.txt", &["Hello", "there"]);
}

#[test]
fn hello1_no_newline() {
    run("hello1.n.txt", &["Hello  there", "-n"]);
}

#[test]
fn hello2_no_newline() {
    run("hello2.n.txt", &["-n", "Hello", "there"]);
}
