use assert_cmd::Command;
use predicates::prelude::predicate::str::diff;
use std::env::temp_dir;
use std::fs;
use uuid::Uuid;

#[test]
fn simple_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "woop"])
        .write_stdin("$1")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn simple_index_missing() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("$1")
        .assert()
        .success()
        .stdout(diff(""));
}

#[test]
fn simple_index_no_unset() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("-u")
        .write_stdin("$1")
        .assert()
        .failure()
        .stderr(diff("1:0 '1' is unset"));
}

#[test]
fn simple_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn simple_named_missing() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff(""));
}

#[test]
fn simple_named_no_unset() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("-u")
        .write_stdin("$VAR")
        .assert()
        .failure()
        .stderr(diff("1:0 'VAR' is unset"));
}

#[test]
fn simple_env() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .env("VAR", "woop")
        .write_stdin("$VAR")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn default_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${1-default}")
        .assert()
        .success()
        .stdout(diff("default"));
}

#[test]
fn default_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${VAR-default}")
        .assert()
        .success()
        .stdout(diff("default"));
}

#[test]
fn default_pattern() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "DEF=woop"])
        .write_stdin("${VAR-$DEF}")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn default_named_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .write_stdin("${VAR:-default}")
        .assert()
        .success()
        .stdout(diff("default"));
}

#[test]
fn alt_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "woop"])
        .write_stdin("${1+alt}")
        .assert()
        .success()
        .stdout(diff("alt"));
}

#[test]
fn alt_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("${VAR+alt}")
        .assert()
        .success()
        .stdout(diff("alt"));
}

#[test]
fn alt_pattern() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .args(&["-v", "ALT=wawawa"])
        .write_stdin("${VAR+$ALT}")
        .assert()
        .success()
        .stdout(diff("wawawa"));
}

#[test]
fn alt_index_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("--")
        .write_stdin("${1:+alt}")
        .assert()
        .success()
        .stdout(diff(""));
}

#[test]
fn alt_named_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .write_stdin("${VAR:+alt}")
        .assert()
        .success()
        .stdout(diff(""));
}

#[test]
fn alt_pattern_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .args(&["-v", "ALT=wawawa"])
        .write_stdin("${VAR:+$ALT}")
        .assert()
        .success()
        .stdout(diff(""));
}

#[test]
fn error_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${1?msg}")
        .assert()
        .failure()
        .stderr(diff("1:0 msg"));
}

#[test]
fn error_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${VAR?msg}")
        .assert()
        .failure()
        .stderr(diff("1:0 msg"));
}

#[test]
fn error_index_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("--")
        .write_stdin("${1:?msg}")
        .assert()
        .failure()
        .stderr(diff("1:0 msg"));
}

#[test]
fn error_named_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .write_stdin("${VAR:?msg}")
        .assert()
        .failure()
        .stderr(diff("1:0 msg"));
}

#[test]
fn error_no_message() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${VAR?}")
        .assert()
        .failure()
        .stderr(diff("1:0 'VAR' is unset"));
}

#[test]
fn error_no_message_no_empty() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR="])
        .write_stdin("${VAR:?}")
        .assert()
        .failure()
        .stderr(diff("1:0 'VAR' is unset or empty"));
}

#[test]
fn len_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "four"])
        .write_stdin("${#1}")
        .assert()
        .success()
        .stdout(diff("4"));
}

#[test]
fn len_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=four"])
        .write_stdin("${#VAR}")
        .assert()
        .success()
        .stdout(diff("4"));
}

#[test]
fn len_missing() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .write_stdin("${#VAR}")
        .assert()
        .success()
        .stdout(diff("0"));
}

#[test]
fn arity() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["--", "one", "two"])
        .write_stdin("${#}")
        .assert()
        .success()
        .stdout(diff("2"));
}

#[test]
fn ref_index() {
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
fn ref_named() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR1=VAR2"])
        .args(&["-v", "VAR2=woop"])
        .write_stdin("${!VAR1}")
        .assert()
        .success()
        .stdout(diff("woop"));
}

#[test]
fn input_file() {
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
fn output_file() {
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
fn var_file() {
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
fn missing_close_brace() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("${VAR")
        .assert()
        .failure()
        .stderr(diff("1:6 Invalid param, unexpected EOF"));
}

#[test]
fn unexpected_token() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "VAR=woop"])
        .write_stdin("${VAR-:def}")
        .assert()
        .failure()
        .stderr(diff("1:8 Unexpected token ':'"));
}

#[test]
fn multiline() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .args(&["-v", "DEF=def"])
        .args(&["--", "jkl"])
        .write_stdin("abc$DEF\nghi$1")
        .assert()
        .success()
        .stdout(diff("abcdef\nghijkl"));
}
