use std::collections::HashMap;
use xpanda::{Error, Xpanda};

#[test]
fn simple_index() {
    let positional_vars = vec![String::from("woop")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "$1";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn simple_index_missing() {
    let xpanda = Xpanda::default();
    let input = "pre $1 post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre  post")));
}

#[test]
fn simple_index_text() {
    let positional_vars = vec![String::from("woop")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "pre $1 post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre woop post")));
}

#[test]
fn simple_index_no_unset() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    let input = "$1";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("'1' is unset"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn simple_index_all() {
    let positional_vars = vec![String::from("first"), String::from("second")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "$0";

    assert_eq!(xpanda.expand(input), Ok(String::from("first second")));
}

#[test]
fn simple_named() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "$VAR";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn simple_named_missing() {
    let xpanda = Xpanda::default();
    let input = "pre $VAR post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre  post")));
}

#[test]
fn simple_named_text() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "pre $VAR post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre woop post")));
}

#[test]
fn simple_named_no_unset() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    let input = "$VAR";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("'VAR' is unset"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn braced_index() {
    let positional_vars = vec![String::from("woop")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${1}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn braced_index_text() {
    let positional_vars = vec![String::from("woop")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "pre ${1} post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre woop post")));
}

#[test]
fn braced_named() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn braced_named_text() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "pre ${VAR} post";

    assert_eq!(xpanda.expand(input), Ok(String::from("pre woop post")));
}

#[test]
fn default_index() {
    let xpanda = Xpanda::default();
    let input = "${1-default}";

    assert_eq!(xpanda.expand(input), Ok(String::from("default")));
}

#[test]
fn default_named() {
    let xpanda = Xpanda::default();
    let input = "${VAR-default}";

    assert_eq!(xpanda.expand(input), Ok(String::from("default")));
}

#[test]
fn default_pattern() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("DEF"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR-$DEF}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn default_index_no_empty() {
    let positional_vars = vec![(String::from(""))];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${1:-default}";

    assert_eq!(xpanda.expand(input), Ok(String::from("default")));
}

#[test]
fn default_named_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:-default}";

    assert_eq!(xpanda.expand(input), Ok(String::from("default")));
}

#[test]
fn default_pattern_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    named_vars.insert(String::from("DEF"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:-$DEF}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn alt_index() {
    let positional_vars = vec![String::from("woop")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${1+alt}";

    assert_eq!(xpanda.expand(input), Ok(String::from("alt")));
}

#[test]
fn alt_named() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR+alt}";

    assert_eq!(xpanda.expand(input), Ok(String::from("alt")));
}

#[test]
fn alt_pattern() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    named_vars.insert(String::from("ALT"), String::from("alt"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR+$ALT}";

    assert_eq!(xpanda.expand(input), Ok(String::from("alt")));
}

#[test]
fn alt_index_no_empty() {
    let positional_vars = vec![String::from("")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${1:+alt}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn alt_named_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:+alt}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn alt_pattern_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    named_vars.insert(String::from("ALT"), String::from("alt"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:+$ALT}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn error_index() {
    let xpanda = Xpanda::default();
    let input = "${1?msg}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("msg"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn error_named() {
    let xpanda = Xpanda::default();
    let input = "${VAR?msg}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("msg"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn error_index_no_empty() {
    let positional_vars = vec![(String::from(""))];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${1:?msg}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("msg"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn error_named_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${1:?msg}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("msg"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn error_no_message() {
    let xpanda = Xpanda::default();
    let input = "${VAR?}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("'VAR' is unset"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn error_no_message_no_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:?}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("'VAR' is unset or empty"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn len_index() {
    let positional_vars = vec![String::from("four")];
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .build();
    let input = "${#1}";

    assert_eq!(xpanda.expand(input), Ok(String::from("4")));
}

#[test]
fn len_named() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("four"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${#VAR}";

    assert_eq!(xpanda.expand(input), Ok(String::from("4")));
}

#[test]
fn len_missing() {
    let xpanda = Xpanda::default();
    let input = "${#VAR}";

    assert_eq!(xpanda.expand(input), Ok(String::from("0")));
}

#[test]
fn len_no_unset() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    let input = "${#VAR}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("'VAR' is unset"),
            line: 1,
            col: 1
        })
    );
}

#[test]
fn missing_close_brace() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("Invalid param, unexpected EOF"),
            line: 1,
            col: 6
        })
    );
}

#[test]
fn unexpected_token() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR-:def}";

    assert_eq!(
        xpanda.expand(input),
        Err(Error {
            message: String::from("Unexpected token ':'"),
            line: 1,
            col: 7
        })
    );
}

#[test]
fn multiline() {
    let positional_vars = vec![(String::from("wawawa"))];
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder()
        .with_positional_vars(positional_vars)
        .with_named_vars(named_vars)
        .build();
    let input = "line 1 $1\n${VAR} line 2";

    assert_eq!(
        xpanda.expand(input),
        Ok(String::from("line 1 wawawa\nwoop line 2"))
    );
}

#[test]
fn uppercase_first() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR^}";

    assert_eq!(xpanda.expand(input), Ok(String::from("Woop")));
}

#[test]
fn uppercase_first_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR^}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn uppercase_all() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR^^}";

    assert_eq!(xpanda.expand(input), Ok(String::from("WOOP")));
}

#[test]
fn lowercase_first() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("WOOP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR,}";

    assert_eq!(xpanda.expand(input), Ok(String::from("wOOP")));
}

#[test]
fn lowercase_first_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR,}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn lowercase_all() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("WOOP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR,,}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn reverse_case_first() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("wOoP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR~}";

    assert_eq!(xpanda.expand(input), Ok(String::from("WOoP")));
}

#[test]
fn reverse_case_first_empty() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from(""));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR~}";

    assert_eq!(xpanda.expand(input), Ok(String::from("")));
}

#[test]
fn reverse_case_all() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("wOoP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR~~}";

    assert_eq!(xpanda.expand(input), Ok(String::from("WoOp")));
}

#[test]
fn syntax_error() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("wOoP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();

    assert_eq!(
        xpanda.expand("${VAR"),
        Err(Error {
            message: String::from("Invalid param, unexpected EOF"),
            line: 1,
            col: 6,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR-"),
        Err(Error {
            message: String::from("Unexpected EOF"),
            line: 1,
            col: 7,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR "),
        Err(Error {
            message: String::from("Invalid param, unexpected token \" \""),
            line: 1,
            col: 6,
        })
    );
    assert_eq!(
        xpanda.expand("${#"),
        Err(Error {
            message: String::from("Expected identifier or close brace, found EOF"),
            line: 1,
            col: 4,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR-:def}"),
        Err(Error {
            message: String::from("Unexpected token ':'"),
            line: 1,
            col: 7,
        })
    );
}
