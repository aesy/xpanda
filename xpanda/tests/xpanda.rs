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
            message: String::from("Invalid param, unexpected EOF"),
            line: 1,
            col: 7,
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
        xpanda.expand("${VAR:1:a}"),
        Err(Error {
            message: String::from("Invalid number: \"a\""),
            line: 1,
            col: 10,
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
            message: String::from("missing authentication"),
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
