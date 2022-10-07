use crate::position::Position;
use crate::str_read::StrRead;
use crate::token::Token;

pub struct Lexer<'a> {
    reader: StrRead<'a>,
    previous_token: Option<Token<'a>>,
    nesting_level: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader: StrRead::new(source),
            previous_token: None,
            nesting_level: 0,
        }
    }

    pub const fn into_iter(mut self) -> IterMut<'a> {
        IterMut::new(self)
    }

    pub fn next_token(&mut self) -> Option<(Token<'a>, Position)> {
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

        self.previous_token = token.clone();

        token.map(|token| (token, self.reader.position().clone()))
    }

    fn read_text(&mut self) -> Option<Token<'a>> {
        let mut slices = Vec::new();

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

        if slices.is_empty() {
            None
        } else {
            let text = String::from_iter(slices);
            Some(Token::Text(text))
        }
    }

    fn read_param(&mut self) -> Option<Token<'a>> {
        let next_char = self.reader.peek_char()?;
        let can_be_identifier = matches!(
            self.previous_token,
            Some(Token::DollarSign | Token::OpenBrace | Token::PoundSign | Token::ExclamationMark)
        );
        let mut is_escaped = self.reader.peek_count(2) == "$$";
        let token = match next_char {
            '$' if !is_escaped => {
                self.reader.consume_char();
                Token::DollarSign
            },
            '{' => {
                self.reader.consume_char();
                Token::OpenBrace
            },
            '}' => {
                self.reader.consume_char();
                Token::CloseBrace
            },
            '!' => {
                self.reader.consume_char();
                Token::ExclamationMark
            },
            ':' => {
                self.reader.consume_char();
                Token::Colon
            },
            '-' => {
                self.reader.consume_char();
                Token::Dash
            },
            '+' => {
                self.reader.consume_char();
                Token::Plus
            },
            '?' => {
                self.reader.consume_char();
                Token::QuestionMark
            },
            '#' => {
                self.reader.consume_char();
                Token::PoundSign
            },
            c if can_be_identifier && c.is_numeric() => {
                let text = self.reader.consume_while(char::is_numeric);
                let number = text.parse().unwrap_or(0);
                Token::Index(number)
            },
            c if can_be_identifier && (c.is_alphanumeric() || c == '_') => {
                let text = self
                    .reader
                    .consume_while(|c| c.is_alphanumeric() || c == '_');
                Token::Identifier(text)
            },
            _ => {
                if is_escaped {
                    self.reader.consume_char();
                }

                let text = self.reader.consume_while(|c| c != '}' && c != '\n');

                if text.is_empty() {
                    return None;
                }

                Token::Text(String::from(text))
            },
        };

        Some(token)
    }
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
    type Item = (Token<'a>, Position);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token()
    }
}
