use std::collections::HashMap;
use xpanda::{Error, ErrorKind, Xpanda};

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
            kind: ErrorKind::Unset {
                name: String::from("1"),
            },
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
            kind: ErrorKind::Unset {
                name: String::from("VAR"),
            },
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
            kind: ErrorKind::UserMessage {
                message: String::from("msg"),
            },
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
            kind: ErrorKind::UserMessage {
                message: String::from("msg"),
            },
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
            kind: ErrorKind::UserMessage {
                message: String::from("msg"),
            },
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
            kind: ErrorKind::UserMessage {
                message: String::from("msg"),
            },
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
            kind: ErrorKind::Unset {
                name: String::from("VAR"),
            },
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
            kind: ErrorKind::UnsetOrEmpty {
                name: String::from("VAR"),
            },
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
            kind: ErrorKind::Unset {
                name: String::from("VAR"),
            },
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
            kind: ErrorKind::InvalidParamEof,
            line: 1,
            col: 6
        })
    );
}

#[test]
fn default_with_colon_text() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR-:def}";

    assert_eq!(xpanda.expand(input), Ok(String::from("woop")));
}

#[test]
fn default_with_colon_text_unset() {
    let xpanda = Xpanda::default();
    let input = "${VAR-:def}";

    assert_eq!(xpanda.expand(input), Ok(String::from(":def")));
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
fn substring() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:2:4}";

    assert_eq!(xpanda.expand(input), Ok(String::from("op w")));
}

#[test]
fn substring_no_length() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:4}";

    assert_eq!(xpanda.expand(input), Ok(String::from(" woop")));
}

#[test]
fn substring_negative_length() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();
    let input = "${VAR:1:-1}";

    assert_eq!(xpanda.expand(input), Ok(String::from("oop woo")));
}

#[test]
fn substring_negative_offset() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();

    let input = "${VAR:(-3)}";
    assert_eq!(xpanda.expand(input), Ok(String::from("oop")));

    let input = "${VAR: -3}";
    assert_eq!(xpanda.expand(input), Ok(String::from("oop")));

    let input = "${VAR: -3: 2}";
    assert_eq!(xpanda.expand(input), Ok(String::from("oo")));
}

#[test]
fn syntax_error() {
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAR"), String::from("wOoP"));
    let xpanda = Xpanda::builder().with_named_vars(named_vars).build();

    assert_eq!(
        xpanda.expand("${VAR"),
        Err(Error {
            kind: ErrorKind::InvalidParamEof,
            line: 1,
            col: 6,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR-"),
        Err(Error {
            kind: ErrorKind::UnexpectedEof,
            line: 1,
            col: 7,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR "),
        Err(Error {
            kind: ErrorKind::InvalidParamEof,
            line: 1,
            col: 7,
        })
    );
    assert_eq!(
        xpanda.expand("${#"),
        Err(Error {
            kind: ErrorKind::Expected {
                expected: String::from("identifier or close brace"),
                found: String::from("EOF"),
            },
            line: 1,
            col: 4,
        })
    );
    assert_eq!(
        xpanda.expand("${VAR:1:a}"),
        Err(Error {
            kind: ErrorKind::InvalidNumber {
                value: String::from("a"),
            },
            line: 1,
            col: 9,
        })
    );
}

#[test]
fn prefix_removal_lazy() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("aaabbbccc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#a*b}"), Ok(String::from("bbccc")));
}

#[test]
fn prefix_removal_greedy() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("aaabbbccc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR##a*b}"), Ok(String::from("ccc")));
}

#[test]
fn prefix_removal_no_match() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("hello"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#xyz}"), Ok(String::from("hello")));
}

#[test]
fn suffix_removal_lazy() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("aaabbbccc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR%b*c}"), Ok(String::from("aaabb")));
}

#[test]
fn suffix_removal_greedy() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("aaabbbccc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR%%b*c}"), Ok(String::from("aaa")));
}

#[test]
fn suffix_removal_path_extension() {
    let mut vars = HashMap::new();
    vars.insert(String::from("FILE"), String::from("photo.tar.gz"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${FILE%.*}"), Ok(String::from("photo.tar")));
    assert_eq!(xpanda.expand("${FILE%%.*}"), Ok(String::from("photo")));
}

#[test]
fn prefix_removal_path_basename() {
    let mut vars = HashMap::new();
    vars.insert(String::from("PATH"), String::from("/usr/local/bin/xpanda"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${PATH##*/}"), Ok(String::from("xpanda")));
}

#[test]
fn replace_first() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("foo bar foo"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${VAR/foo/baz}"),
        Ok(String::from("baz bar foo"))
    );
}

#[test]
fn replace_all() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("foo bar foo"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${VAR//foo/baz}"),
        Ok(String::from("baz bar baz"))
    );
}

#[test]
fn replace_with_glob() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("foo123bar"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR/[0-9]*/X}"), Ok(String::from("fooX")));
}

#[test]
fn replace_empty_replacement_deletes() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("foo bar"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR// /}"), Ok(String::from("foobar")));
}

#[test]
fn replace_inline_param() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("hello world"));
    vars.insert(String::from("NEEDLE"), String::from("world"));
    vars.insert(String::from("REPL"), String::from("rust"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${VAR/$NEEDLE/$REPL}"),
        Ok(String::from("hello rust"))
    );
}

#[test]
fn quoted_default_with_specials() {
    let xpanda = Xpanda::default();
    assert_eq!(xpanda.expand("${VAR-\"a: b\"}"), Ok(String::from("a: b")));
}

#[test]
fn single_quoted_default_suppresses_expansion() {
    let mut vars = HashMap::new();
    vars.insert(String::from("X"), String::from("expanded"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR-'$X'}"), Ok(String::from("$X")));
}

#[test]
fn double_quoted_default_expands() {
    let mut vars = HashMap::new();
    vars.insert(String::from("X"), String::from("expanded"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR-\"$X\"}"), Ok(String::from("expanded")));
}

#[test]
fn quoted_glob_is_literal() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("*xyz"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#\"*\"}"), Ok(String::from("xyz")));
}

#[test]
fn inline_param_in_default() {
    let mut vars = HashMap::new();
    vars.insert(String::from("OTHER"), String::from("X"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${VAR-pre$OTHER post}"),
        Ok(String::from("preX post"))
    );
}

#[test]
fn interpolated_error_message() {
    let mut vars = HashMap::new();
    vars.insert(String::from("WHAT"), String::from("authentication"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${MISSING?missing $WHAT}"),
        Err(Error {
            kind: ErrorKind::UserMessage {
                message: String::from("missing authentication"),
            },
            line: 1,
            col: 1,
        })
    );
}

#[test]
fn inline_param_in_alt() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("set"));
    vars.insert(String::from("ALT"), String::from("alt-value"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(
        xpanda.expand("${VAR+pre-$ALT-post}"),
        Ok(String::from("pre-alt-value-post"))
    );
}

#[test]
fn substring_paren_positive() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR:(2):(4)}"), Ok(String::from("op w")));
}

#[test]
fn substring_paren_with_spaces() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("woop woop"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR:( -3 ):( 2 )}"), Ok(String::from("oo")));
}

#[test]
fn arity_none() {
    let xpanda = Xpanda::default();
    assert_eq!(xpanda.expand("${#}"), Ok(String::from("0")));
}

#[test]
fn arity_multiple() {
    let vars = vec![String::from("a"), String::from("b"), String::from("c")];
    let xpanda = Xpanda::builder().with_positional_vars(vars).build();
    assert_eq!(xpanda.expand("${#}"), Ok(String::from("3")));
}

#[test]
fn arity_in_text() {
    let vars = vec![String::from("only")];
    let xpanda = Xpanda::builder().with_positional_vars(vars).build();
    assert_eq!(xpanda.expand("got ${#} arg"), Ok(String::from("got 1 arg")));
}

#[test]
fn indirect_ref_hit() {
    let mut vars = HashMap::new();
    vars.insert(String::from("NAME"), String::from("TARGET"));
    vars.insert(String::from("TARGET"), String::from("hit"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${!NAME}"), Ok(String::from("hit")));
}

#[test]
fn indirect_ref_target_unset() {
    let mut vars = HashMap::new();
    vars.insert(String::from("NAME"), String::from("MISSING"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${!NAME}"), Ok(String::new()));
}

#[test]
fn indirect_ref_missing_no_unset() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    assert!(xpanda.expand("${!NAME}").is_err());
}

#[test]
fn dollar_escape_top_level() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("value"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("$$VAR"), Ok(String::from("$VAR")));
    assert_eq!(xpanda.expand("$${VAR}"), Ok(String::from("${VAR}")));
    assert_eq!(
        xpanda.expand("pre $$VAR post"),
        Ok(String::from("pre $VAR post"))
    );
}

#[test]
fn dollar_escape_in_default_body() {
    let xpanda = Xpanda::default();
    assert_eq!(xpanda.expand("${VAR-$$text}"), Ok(String::from("$text")));
}

#[test]
fn whitespace_in_braces() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${ VAR }"), Ok(String::from("woop")));
    assert_eq!(xpanda.expand("${VAR }"), Ok(String::from("woop")));
    assert_eq!(xpanda.expand("${ VAR}"), Ok(String::from("woop")));
}

#[test]
fn whitespace_in_default_body_is_data() {
    let xpanda = Xpanda::default();
    assert_eq!(xpanda.expand("${VAR-  foo  }"), Ok(String::from("  foo  ")));
}

#[test]
fn no_unset_substring() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    assert!(xpanda.expand("${VAR:2}").is_err());
    assert!(xpanda.expand("${VAR:2:4}").is_err());
}

#[test]
fn no_unset_prefix_removal() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    assert!(xpanda.expand("${VAR#*e}").is_err());
    assert!(xpanda.expand("${VAR##*e}").is_err());
}

#[test]
fn no_unset_suffix_removal() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    assert!(xpanda.expand("${VAR%e*}").is_err());
    assert!(xpanda.expand("${VAR%%e*}").is_err());
}

#[test]
fn no_unset_replace() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    assert!(xpanda.expand("${VAR/a/b}").is_err());
    assert!(xpanda.expand("${VAR//a/b}").is_err());
}

#[test]
fn glob_question_mark() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("abc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#?}"), Ok(String::from("bc")));
}

#[test]
fn glob_character_class() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("abc"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#[abc]}"), Ok(String::from("bc")));
    assert_eq!(xpanda.expand("${VAR#[xyz]}"), Ok(String::from("abc")));
}

#[test]
fn glob_range() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("m"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand("${VAR#[a-z]}"), Ok(String::new()));
    assert_eq!(xpanda.expand("${VAR#[A-Z]}"), Ok(String::from("m")));
}

#[test]
fn glob_backslash_escape() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("*rest"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();
    assert_eq!(xpanda.expand(r"${VAR#\*}"), Ok(String::from("rest")));
}

fn with_var(value: &str) -> Xpanda {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from(value));

    Xpanda::builder().with_named_vars(vars).build()
}

fn expand_ok(xpanda: &Xpanda, input: &str) -> String {
    xpanda.expand(input).unwrap()
}

#[test]
fn identifier_like_text_in_bodies_is_kept() {
    let xpanda = with_var("abcdef");

    assert_eq!(expand_ok(&xpanda, "${VAR#abc}"), "def");
    assert_eq!(expand_ok(&xpanda, "${VAR##abc}"), "def");
    assert_eq!(expand_ok(&with_var("123"), "${VAR#1}"), "23");
    assert_eq!(expand_ok(&with_var("007x"), "${VAR#007}"), "x");
    assert_eq!(expand_ok(&xpanda, "${UNSET-#foo}"), "#foo");
    assert_eq!(expand_ok(&xpanda, "${UNSET-a!foo}"), "a!foo");
    assert_eq!(expand_ok(&xpanda, "${UNSET-#007}"), "#007");
}

#[test]
fn substring_counts_characters() {
    let xpanda = with_var("åäö");

    assert_eq!(expand_ok(&xpanda, "${VAR:1}"), "äö");
    assert_eq!(expand_ok(&xpanda, "${VAR:1:1}"), "ä");
    assert_eq!(expand_ok(&xpanda, "${VAR: -1}"), "ö");
    assert_eq!(expand_ok(&xpanda, "${VAR:0:-1}"), "åä");
    assert_eq!(expand_ok(&xpanda, "${#VAR}"), "3");
}

#[test]
fn substring_edge_cases_match_bash() {
    let xpanda = with_var("abc");

    assert_eq!(expand_ok(&xpanda, "${VAR: -5}"), "");
    assert_eq!(expand_ok(&xpanda, "${VAR:(-5)}"), "");
    assert_eq!(expand_ok(&xpanda, "${VAR:1:}"), "");
}

#[test]
fn with_env_vars_sources_env_vars() {
    let xpanda = Xpanda::builder().with_env_vars().build();

    assert_eq!(
        xpanda.expand("$CARGO_PKG_NAME"),
        Ok(String::from(env!("CARGO_PKG_NAME")))
    );
}

#[test]
fn error_display() {
    let error = Xpanda::default().expand("abc ${VAR?}").unwrap_err();

    assert_eq!(error.to_string(), "1:5 'VAR' is unset");

    let boxed: Box<dyn std::error::Error> = Box::new(error);

    assert_eq!(boxed.to_string(), "1:5 'VAR' is unset");
}

#[test]
fn nested_params_in_double_quotes() {
    let mut vars = HashMap::new();
    vars.insert(String::from("B"), String::from("b"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();

    assert_eq!(expand_ok(&xpanda, r#"${U-"${B}x"}"#), "bx");
    assert_eq!(expand_ok(&xpanda, r#"${U-"${C-y}"}"#), "y");
    assert_eq!(expand_ok(&xpanda, r#"${U-"${C-"x y"} z"}"#), "x y z");
    assert_eq!(expand_ok(&xpanda, r#"${U-"$B c"}"#), "b c");
    assert_eq!(expand_ok(&xpanda, r#"${U-"$$B"}"#), "$B");
}

#[test]
fn invalid_indexes_are_unset() {
    let xpanda = Xpanda::builder()
        .with_positional_vars(vec![String::from("one")])
        .build();

    assert_eq!(expand_ok(&xpanda, "[$99999999999999999999]"), "[]");
    assert_eq!(expand_ok(&xpanda, "[$٣]"), "[]");
}

#[test]
fn indirect_ref_to_positional() {
    let xpanda = Xpanda::builder()
        .with_positional_vars(vec![String::from("2"), String::from("two")])
        .build();

    assert_eq!(expand_ok(&xpanda, "${!1}"), "two");
}

#[test]
fn literal_braces_in_bodies() {
    let xpanda = Xpanda::builder().build();

    assert_eq!(expand_ok(&xpanda, "${U-{x}}"), "{x}");
    assert_eq!(expand_ok(&xpanda, "${U-{x}y}"), "{xy}");
    assert_eq!(expand_ok(&xpanda, r#"${U:-{"a":1}}"#), "{a:1}");
}

#[test]
fn newlines_in_bodies() {
    let xpanda = Xpanda::builder().build();

    assert_eq!(expand_ok(&xpanda, "${U-a\nb}"), "a\nb");
}

#[test]
fn replace_on_empty_value() {
    let xpanda = with_var("");

    assert_eq!(expand_ok(&xpanda, "${VAR/*/x}"), "x");
    assert_eq!(expand_ok(&xpanda, "${VAR//*/x}"), "x");
    assert_eq!(expand_ok(&xpanda, "${VAR/a*/x}"), "");
    assert_eq!(expand_ok(&xpanda, "${VAR/$VAR/x}"), "");
}

#[test]
fn glob_negated_class() {
    let xpanda = with_var("bcd");

    assert_eq!(expand_ok(&xpanda, "${VAR#[!a]}"), "cd");
    assert_eq!(expand_ok(&xpanda, "${VAR#[^a]}"), "cd");
    assert_eq!(expand_ok(&xpanda, "${VAR#[!b]}"), "bcd");
}

#[test]
fn glob_unterminated_class_is_literal() {
    assert_eq!(expand_ok(&with_var("[a"), "${VAR#[}"), "a");
}

#[test]
fn whitespace_around_prefix_operators() {
    let mut vars = HashMap::new();
    vars.insert(String::from("VAR"), String::from("OTHER"));
    vars.insert(String::from("OTHER"), String::from("woop"));
    let xpanda = Xpanda::builder().with_named_vars(vars).build();

    assert_eq!(expand_ok(&xpanda, "${ #VAR }"), "5");
    assert_eq!(expand_ok(&xpanda, "${#VAR }"), "5");
    assert_eq!(expand_ok(&xpanda, "${ !VAR }"), "woop");
}

#[test]
fn glob_many_stars_finish_quickly() {
    let xpanda = with_var(&"a".repeat(200));
    let input = format!("${{VAR#{}b}}", "*a".repeat(50));

    assert_eq!(xpanda.expand(&input), Ok("a".repeat(200)));
}

#[test]
fn too_deep_nesting_is_an_error() {
    let xpanda = Xpanda::builder().build();
    let nested = |depth: usize| "${A-".repeat(depth) + "x" + &"}".repeat(depth);

    assert_eq!(xpanda.expand(&nested(256)), Ok(String::from("x")));
    assert_eq!(
        xpanda.expand(&nested(200_000)).unwrap_err().kind,
        ErrorKind::NestingTooDeep { max_depth: 256 }
    );
}

#[test]
fn eval_error_positions() {
    let xpanda = Xpanda::builder().no_unset(true).build();
    let position = |input: &str| {
        let error = xpanda.expand(input).unwrap_err();

        (error.line, error.col)
    };

    assert_eq!(position("abc $VAR"), (1, 5));
    assert_eq!(position("a\n  ${VAR?msg}"), (2, 3));
    assert_eq!(position("abc ${X-${VAR?msg}}"), (1, 9));
    assert_eq!(position("åäö $VAR"), (1, 5));
}

#[test]
fn parse_error_positions() {
    let xpanda = Xpanda::builder().build();
    let position = |input: &str| {
        let error = xpanda.expand(input).unwrap_err();

        (error.line, error.col)
    };

    assert_eq!(position("abc $"), (1, 6));
    assert_eq!(position("abc ${VAR=x}"), (1, 10));
    assert_eq!(position("a\n${#V x}"), (2, 6));
}
