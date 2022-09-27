use assert_cmd::Command;
use predicates::prelude::predicate::str::diff;

#[test]
fn simple_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("woop")
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
        .arg("-v VAR=")
        .write_stdin("${VAR:-default}")
        .assert()
        .success()
        .stdout(diff("default"));
}

#[test]
fn alt_index() {
    Command::cargo_bin("xpanda-cli")
        .unwrap()
        .arg("woop")
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
        .arg("")
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
        .arg("")
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
        .arg("four")
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
