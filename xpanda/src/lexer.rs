use crate::position::Position;
use crate::str_read::StrRead;
use crate::token::Token;
use std::borrow::Cow;

pub struct Lexer<'a> {
    reader: StrRead<'a>,
    previous_token: Option<Token<'a>>,
    after_prefix_operator: bool,
    nesting_level: usize,
    double_quote_levels: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader: StrRead::new(source),
            previous_token: None,
            after_prefix_operator: false,
            nesting_level: 0,
            double_quote_levels: Vec::new(),
        }
    }

    pub const fn into_iter(self) -> IterMut<'a> {
        IterMut::new(self)
    }

    /// Returns the next token along with its start and end positions.
    pub fn next_token(&mut self) -> Option<(Token<'a>, Position, Position)> {
        if self.previous_token == Some(Token::OpenBrace) {
            self.reader.consume_while(|c| c == ' ');
        }

        let start = self.reader.position().clone();
        let is_param = self.nesting_level > 0 || self.previous_token == Some(Token::DollarSign);

        let token = if is_param {
            self.read_param()
        } else {
            let next_char = self.reader.peek_char();
            let is_escaped = self.reader.peek_count(2) == "$$";

            if next_char == Some('$') && !is_escaped {
                self.read_param()
            } else {
                self.read_text()
            }
        };

        self.nesting_level = match token {
            Some(Token::OpenBrace) => self.nesting_level.saturating_add(1),
            Some(Token::CloseBrace) => self.nesting_level.saturating_sub(1),
            _ => self.nesting_level,
        };

        self.after_prefix_operator = self.previous_token == Some(Token::OpenBrace)
            && matches!(token, Some(Token::PoundSign | Token::ExclamationMark));
        self.previous_token.clone_from(&token);

        token.map(|token| (token, start, self.reader.position().clone()))
    }

    fn read_text(&mut self) -> Option<Token<'a>> {
        let mut slices: Vec<&'a str> = Vec::new();

        loop {
            let is_escaped = self.reader.peek_count(2) == "$$";

            if is_escaped {
                self.reader.consume_char();
                self.reader.consume_char();
                slices.push("$");
            }

            let text = self.reader.consume_while(|c| c != '$');

            if text.is_empty() {
                break;
            }

            slices.push(text);
        }

        match slices.len() {
            0 => None,
            1 => Some(Token::Text(Cow::Borrowed(slices[0]))),
            _ => Some(Token::Text(Cow::Owned(String::from_iter(slices)))),
        }
    }

    fn read_param(&mut self) -> Option<Token<'a>> {
        if let Some(token) = self.handle_in_double_quote() {
            return Some(token);
        }

        let next_char = self.reader.peek_char()?;
        let in_brace_head =
            self.previous_token == Some(Token::OpenBrace) || self.after_prefix_operator;
        let can_be_identifier = in_brace_head || self.previous_token == Some(Token::DollarSign);
        let is_escaped = self.reader.peek_count(2) == "$$";

        // Only `${` opens a param. Any other `{` is plain text and doesn't
        // nest, as in Bash.
        if next_char == '{' && self.previous_token != Some(Token::DollarSign) {
            self.reader.consume_char();
            return Some(Token::Text(Cow::Borrowed("{")));
        }

        if let Some(token) = single_char_token(next_char, is_escaped) {
            self.reader.consume_char();
            return Some(token);
        }

        let token = match next_char {
            '\'' => {
                self.reader.consume_char();

                let text = self.reader.consume_while(|c| c != '\'');

                if self.reader.peek_char() == Some('\'') {
                    self.reader.consume_char();
                }

                Token::QuotedText(Cow::Borrowed(text))
            },
            '"' => {
                self.reader.consume_char();
                self.double_quote_levels.push(self.nesting_level);

                let text = self.reader.consume_while(|c| c != '"' && c != '$');

                if self.reader.peek_char() == Some('"') {
                    self.reader.consume_char();
                    self.double_quote_levels.pop();
                }

                Token::QuotedText(Cow::Borrowed(text))
            },
            c if can_be_identifier && c.is_ascii_digit() => {
                let text = self.reader.consume_while(|c| c.is_ascii_digit());
                // An index too large to parse can't refer to any provided
                // variable, so it's unset.
                let number = text.parse().unwrap_or(usize::MAX);

                if in_brace_head {
                    self.reader.consume_while(|c| c == ' ');
                }

                Token::Index(number)
            },
            c if can_be_identifier && (c.is_alphanumeric() || c == '_') => {
                let text = self
                    .reader
                    .consume_while(|c| c.is_alphanumeric() || c == '_');

                if in_brace_head {
                    self.reader.consume_while(|c| c == ' ');
                }

                Token::Identifier(text)
            },
            _ => {
                if is_escaped {
                    self.reader.consume_char();
                    self.reader.consume_char();

                    let text = self.reader.consume_while(|c| !is_brace_operator(c));
                    let mut buf = String::with_capacity(1 + text.len());

                    buf.push('$');
                    buf.push_str(text);

                    return Some(Token::Text(Cow::Owned(buf)));
                }

                let text = self.reader.consume_while(|c| !is_brace_operator(c));

                if text.is_empty() {
                    return None;
                }

                Token::Text(Cow::Borrowed(text))
            },
        };

        Some(token)
    }

    fn handle_in_double_quote(&mut self) -> Option<Token<'a>> {
        let inside_quoted_text = self.double_quote_levels.last() == Some(&self.nesting_level)
            && self.previous_token != Some(Token::DollarSign);

        if !inside_quoted_text {
            return None;
        }

        match self.reader.peek_char()? {
            '"' => {
                self.reader.consume_char();
                self.double_quote_levels.pop();
                None
            },
            '$' if self.reader.peek_count(2) == "$$" => {
                self.reader.consume_char();
                self.reader.consume_char();
                Some(Token::QuotedText(Cow::Borrowed("$")))
            },
            '$' => None,
            _ => {
                let text = self.reader.consume_while(|c| c != '"' && c != '$');
                Some(Token::QuotedText(Cow::Borrowed(text)))
            },
        }
    }
}

const fn single_char_token<'a>(c: char, is_escaped: bool) -> Option<Token<'a>> {
    Some(match c {
        '$' if !is_escaped => Token::DollarSign,
        '{' => Token::OpenBrace,
        '}' => Token::CloseBrace,
        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
        '!' => Token::ExclamationMark,
        ':' => Token::Colon,
        '-' => Token::Dash,
        '+' => Token::Plus,
        '?' => Token::QuestionMark,
        '#' => Token::PoundSign,
        '/' => Token::ForwardSlash,
        '%' => Token::Percent,
        ',' => Token::Comma,
        '^' => Token::Caret,
        '~' => Token::Tilde,
        _ => return None,
    })
}

const fn is_brace_operator(c: char) -> bool {
    matches!(
        c,
        '$' | '{'
            | '}'
            | '('
            | ')'
            | '!'
            | ':'
            | '-'
            | '+'
            | '?'
            | '#'
            | '/'
            | '%'
            | ','
            | '^'
            | '~'
            | '\''
            | '"'
    )
}

pub struct IterMut<'a> {
    lexer: Lexer<'a>,
}

impl<'a> IterMut<'a> {
    const fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Token<'a>, Position, Position);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token()
    }
}
