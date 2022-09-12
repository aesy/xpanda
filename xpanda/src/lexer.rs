use crate::token::Token;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    index: usize,
    previous_token: Option<Token<'a>>,
    is_param: bool,
    brace_level: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.char_indices().peekable();
        let is_param = matches!(chars.peek(), Some((_, '$')));

        Self {
            source,
            chars,
            index: 0,
            previous_token: None,
            is_param,
            brace_level: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let token = if self.is_param {
            self.read_param()
        } else {
            self.read_text()
        };

        match token {
            Some(Token::OpenBrace) => self.brace_level += 1,
            Some(Token::CloseBrace) => self.brace_level -= 1,
            _ => {},
        }

        match token {
            _ if !self.is_param => self.is_param = true,
            Some(Token::DollarSign) => self.is_param = true,
            _ if self.brace_level == 0 => self.is_param = false,
            _ => self.is_param = true,
        }

        self.previous_token = token.clone();

        token
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    fn consume_char(&mut self) -> Option<char> {
        let (i, c) = self.chars.next()?;

        self.index = i + c.len_utf8();

        Some(c)
    }

    fn consume_while<P>(&mut self, predicate: P) -> &'a str
    where
        P: Fn(char) -> bool,
    {
        let start = self.index;

        while let Some(c) = self.peek_char() {
            if !predicate(c) {
                break;
            }

            self.consume_char();
        }

        let end = self.index;

        &self.source[start..end]
    }

    fn read_text(&mut self) -> Option<Token<'a>> {
        let end = if self.is_param { '}' } else { '$' };
        let text = self.consume_while(|c| c != end);

        if text.is_empty() {
            None
        } else {
            Some(Token::Text(text))
        }
    }

    fn read_param(&mut self) -> Option<Token<'a>> {
        let next_token = self.peek_char()?;

        if let Some(Token::Dash | Token::Plus | Token::QuestionMark) = self.previous_token {
            if next_token != '$' && next_token != '}' {
                return self.read_text();
            }
        };

        let token = match next_token {
            '{' => {
                self.consume_char()?;
                Token::OpenBrace
            },
            '}' => {
                self.consume_char()?;
                Token::CloseBrace
            },
            '$' => {
                self.consume_char()?;
                Token::DollarSign
            },
            ':' => {
                self.consume_char()?;
                Token::Colon
            },
            '-' => {
                self.consume_char()?;
                Token::Dash
            },
            '+' => {
                self.consume_char()?;
                Token::Plus
            },
            '?' => {
                self.consume_char()?;
                Token::QuestionMark
            },
            '#' => {
                self.consume_char()?;
                Token::PoundSign
            },
            c if c.is_numeric() => {
                let text = self.consume_while(char::is_numeric);
                let number = text.parse().unwrap_or(0);
                Token::Index(number)
            },
            c if c.is_alphanumeric() || c == '_' => {
                let text = self.consume_while(|c| c.is_alphanumeric() || c == '_');
                Token::Identifier(text)
            },
            _ => self.read_text()?,
        };

        Some(token)
    }
}
