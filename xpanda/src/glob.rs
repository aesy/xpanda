use crate::str_read::StrRead;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
enum Token {
    ExactStr(String),
    // *
    AnyStr,
    // ?
    AnyChar,
    // [..]
    AnyOf(Vec<Enclosed>),
}

#[derive(Debug, Clone)]
enum Enclosed {
    Char(char),
    Range { from: char, to: char },
}

#[derive(Debug, Clone)]
pub struct Glob {
    tokens: Vec<Token>,
}

impl Glob {
    pub fn compile(pattern: &str) -> Result<Self, Error> {
        let mut reader = StrRead::new(pattern);
        let mut tokens: Vec<Token> = Vec::new();

        let push_char = |tokens: &mut Vec<Token>, c: char| {
            if let Some(Token::ExactStr(s)) = tokens.last_mut() {
                s.push(c);
            } else {
                tokens.push(Token::ExactStr(c.to_string()));
            }
        };

        while let Some(c) = reader.peek_char() {
            match c {
                '*' => {
                    reader.consume_char();

                    // Collapse runs of `*` into one - they are equivalent.
                    if !matches!(tokens.last(), Some(Token::AnyStr)) {
                        tokens.push(Token::AnyStr);
                    }
                },
                '?' => {
                    reader.consume_char();
                    tokens.push(Token::AnyChar);
                },
                '\\' => {
                    reader.consume_char();
                    match reader.consume_char() {
                        Some(escaped) => push_char(&mut tokens, escaped),
                        None => {
                            return Err(Error {
                                message: String::from("Trailing backslash in pattern"),
                            });
                        },
                    }
                },
                '[' => {
                    reader.consume_char();
                    let mut variants = Vec::new();

                    loop {
                        match reader.peek_char() {
                            Some(']') | None => break,
                            Some(c1) => {
                                reader.consume_char();
                                if reader.peek_char() == Some('-') {
                                    let saved_idx = 1;
                                    reader.consume_char();
                                    match reader.peek_char() {
                                        Some(']') | None => {
                                            variants.push(Enclosed::Char(c1));
                                            variants.push(Enclosed::Char('-'));
                                        },
                                        Some(c2) => {
                                            reader.consume_char();
                                            variants.push(Enclosed::Range { from: c1, to: c2 });
                                        },
                                    }
                                    let _ = saved_idx;
                                } else {
                                    variants.push(Enclosed::Char(c1));
                                }
                            },
                        }
                    }

                    match reader.consume_char() {
                        Some(']') => {},
                        _ => {
                            return Err(Error {
                                message: String::from("Unterminated character class"),
                            });
                        },
                    }

                    tokens.push(Token::AnyOf(variants));
                },
                _ => {
                    reader.consume_char();
                    push_char(&mut tokens, c);
                },
            }
        }

        Ok(Self { tokens })
    }

    /// True iff the entire input matches the pattern.
    pub fn matches(&self, input: &str) -> bool {
        match_lengths(&self.tokens, input)
            .into_iter()
            .any(|n| n == input.len())
    }

    /// Returns `input` with the longest (greedy) or shortest (lazy) prefix
    /// that matches the pattern removed. If no prefix matches the pattern,
    /// `input` is returned unchanged.
    pub fn trim_start<'a>(&self, input: &'a str, greedy: bool) -> &'a str {
        let lengths = match_lengths(&self.tokens, input);
        pick(&lengths, greedy).map_or(input, |n| &input[n..])
    }

    /// Returns `input` with the longest (greedy) or shortest (lazy) suffix
    /// that matches the pattern removed.
    pub fn trim_end<'a>(&self, input: &'a str, greedy: bool) -> &'a str {
        // Walk every UTF-8-safe suffix-start and ask: does the pattern match
        // the entire suffix? Pick the longest (greedy) or shortest (lazy).
        let mut candidates: Vec<usize> = Vec::new();
        let mut i = 0;

        while i <= input.len() {
            if input.is_char_boundary(i) {
                let suffix_len = input.len() - i;

                if match_lengths(&self.tokens, &input[i..])
                    .into_iter()
                    .any(|n| n == suffix_len)
                {
                    candidates.push(suffix_len);
                }
            }

            i += 1;
        }

        pick(&candidates, greedy).map_or(input, |n| &input[..input.len() - n])
    }

    /// Replace the first or all occurrences of any substring matching the
    /// pattern with `replacement`. A "match" is the longest substring at the
    /// current scan position that the pattern matches.
    pub fn replace(&self, input: &str, replacement: &str, all_occurrences: bool) -> String {
        let mut out = String::with_capacity(input.len());
        let mut i = 0;

        while i < input.len() {
            if !input.is_char_boundary(i) {
                i += 1;
                continue;
            }

            let lengths = match_lengths(&self.tokens, &input[i..]);

            if let Some(&n) = lengths.iter().max() {
                if n == 0 {
                    let next = input[i..].chars().next().map_or(1, char::len_utf8);
                    out.push_str(&input[i..i + next]);
                    i += next;
                    continue;
                }

                out.push_str(replacement);

                i += n;

                if !all_occurrences {
                    out.push_str(&input[i..]);
                    return out;
                }
            } else {
                let next = input[i..].chars().next().map_or(1, char::len_utf8);
                out.push_str(&input[i..i + next]);
                i += next;
            }
        }

        out
    }
}

fn pick(lengths: &[usize], greedy: bool) -> Option<usize> {
    if greedy {
        lengths.iter().copied().max()
    } else {
        lengths.iter().copied().min()
    }
}

/// Return the set of prefix byte-lengths of `input` that the token sequence
/// fully consumes. An empty result means no match.
fn match_lengths(tokens: &[Token], input: &str) -> Vec<usize> {
    if tokens.is_empty() {
        return vec![0];
    }

    let mut results: Vec<usize> = Vec::new();
    match &tokens[0] {
        Token::AnyStr => {
            let mut i = 0;

            loop {
                if input.is_char_boundary(i) {
                    for j in match_lengths(&tokens[1..], &input[i..]) {
                        results.push(i + j);
                    }
                }

                if i >= input.len() {
                    break;
                }

                i += 1;
            }
        },
        Token::AnyChar => {
            if let Some(c) = input.chars().next() {
                let n = c.len_utf8();

                for j in match_lengths(&tokens[1..], &input[n..]) {
                    results.push(n + j);
                }
            }
        },
        Token::ExactStr(s) => {
            if input.starts_with(s.as_str()) {
                let n = s.len();

                for j in match_lengths(&tokens[1..], &input[n..]) {
                    results.push(n + j);
                }
            }
        },
        Token::AnyOf(variants) => {
            if let Some(c) = input.chars().next() {
                let matches = variants.iter().any(|v| match v {
                    Enclosed::Char(x) => *x == c,
                    Enclosed::Range { from, to } => *from <= c && c <= *to,
                });

                if matches {
                    let n = c.len_utf8();

                    for j in match_lengths(&tokens[1..], &input[n..]) {
                        results.push(n + j);
                    }
                }
            }
        },
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compile(p: &str) -> Glob {
        Glob::compile(p).unwrap()
    }

    #[test]
    fn empty_pattern_matches_empty() {
        assert!(compile("").matches(""));
        assert!(!compile("").matches("a"));
    }

    #[test]
    fn exact_match() {
        assert!(compile("abc").matches("abc"));
        assert!(!compile("abc").matches("ab"));
        assert!(!compile("abc").matches("abcd"));
    }

    #[test]
    fn star_any() {
        assert!(compile("*").matches(""));
        assert!(compile("*").matches("hello"));
        assert!(compile("a*c").matches("ac"));
        assert!(compile("a*c").matches("abc"));
        assert!(compile("a*c").matches("axxxc"));
        assert!(!compile("a*c").matches("ab"));
    }

    #[test]
    fn qmark_one() {
        assert!(compile("a?c").matches("abc"));
        assert!(compile("a?c").matches("axc"));
        assert!(!compile("a?c").matches("ac"));
        assert!(!compile("a?c").matches("abbc"));
    }

    #[test]
    fn class_and_range() {
        assert!(compile("[abc]").matches("a"));
        assert!(compile("[abc]").matches("b"));
        assert!(!compile("[abc]").matches("d"));
        assert!(compile("[a-z]").matches("m"));
        assert!(!compile("[a-z]").matches("M"));
        assert!(compile("[A-Za-z]").matches("M"));
    }

    #[test]
    fn escape() {
        assert!(compile(r"\*").matches("*"));
        assert!(!compile(r"\*").matches("anything"));
        assert!(compile(r"a\?b").matches("a?b"));
    }

    #[test]
    fn trim_start_greedy_vs_lazy() {
        let glob = compile("a*");
        assert_eq!(glob.trim_start("abcabc", true), "");
        assert_eq!(glob.trim_start("abcabc", false), "bcabc");
    }

    #[test]
    fn trim_start_no_match_returns_input() {
        let glob = compile("x*");
        assert_eq!(glob.trim_start("abc", true), "abc");
    }

    #[test]
    fn trim_end_greedy_vs_lazy() {
        let glob = compile("*c");
        assert_eq!(glob.trim_end("abcabc", true), "");
        assert_eq!(glob.trim_end("abcabc", false), "abcab");
    }

    #[test]
    fn replace_first_and_all() {
        let glob = compile("o");
        assert_eq!(glob.replace("foo bar", "X", false), "fXo bar");
        assert_eq!(glob.replace("foo bar", "X", true), "fXX bar");
    }

    #[test]
    fn replace_with_glob() {
        let glob = compile("b*r");
        assert_eq!(glob.replace("foo bar baz", "X", false), "foo X baz");
    }
}
