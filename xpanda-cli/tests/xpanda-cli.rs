use assert_cmd::Command;
use predicates::prelude::predicate::str::diff;
use std::env::temp_dir;
use std::fs;
use uuid::Uuid;

#[test]
fn positional_var_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "woop"])
        .write_stdin("$1")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn named_var_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "DEF=woop"])
        .write_stdin("${VAR-$DEF}")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn no_unset_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("-u")
        .write_stdin("$VAR")
        .assert()
        .failure()
        .stderr(diff("1:1 'VAR' is unset"));
}

#[test]
fn env_var_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .env("VAR", "woop")
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn var_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${VAR?msg}")
        .assert()
        .failure()
        .stderr(diff("1:1 msg"));
}

#[test]
fn var_unset_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${VAR?}")
        .assert()
        .failure()
        .stderr(diff("1:1 'VAR' is unset"));
}

#[test]
fn var_unset_or_empty_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .write_stdin("${VAR:?}")
        .assert()
        .failure()
        .stderr(diff("1:1 'VAR' is unset or empty"));
}

#[test]
fn arity_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "one", "two"])
        .write_stdin("${#}")
        .assert()
        .success()
        .stdout(diff("2"));
}

#[test]
fn ref_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .args(&["--", "VAR"])
        .write_stdin("${!1}")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn input_file_success() {
    let mut file = temp_dir();
    file.push(Uuid::new_v4().to_string() + "-xpanda-test-input");
    fs::write(&file, "$VAR").unwrap();

    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-i", file.to_str().unwrap()])
        .args(&["-v", "VAR=woop"])
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn output_file_success() {
    let mut file = temp_dir();
    file.push(Uuid::new_v4().to_string() + "-xpanda-test-output");

    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-o", file.to_str().unwrap()])
        .args(&["-v", "VAR=woop"])
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff(""));

    let content = fs::read_to_string(&file).unwrap();
    assert_eq!(content, "woop");
}

#[test]
fn var_file_success() {
    let mut file = temp_dir();
    file.push(Uuid::new_v4().to_string() + "-xpanda-test-vars");
    fs::write(&file, "VAR=woop").unwrap();

    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-f", file.to_str().unwrap()])
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn unexpected_eof_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("${VAR")
        .assert()
        .failure()
        .stderr(diff("1:6 Invalid param, unexpected EOF"));
}

#[test]
fn unexpected_token_error() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("${VAR-:def}")
        .assert()
        .failure()
        .stderr(diff("1:7 Unexpected token ':'"));
}

#[test]
fn multiline_success() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "DEF=def"])
        .args(&["--", "jkl"])
        .write_stdin("abc$DEF\nghi$1")
        .assert()
        .success()
        .stdout(diff("abcdef\nghijkl"));
}
