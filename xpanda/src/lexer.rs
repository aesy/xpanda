use crate::str_read::StrRead;
use crate::token::Token;
use std::borrow::Cow;

pub struct Lexer<'a> {
    reader: StrRead<'a>,
    previous_token: Option<Token>,
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

    pub fn next_token(&mut self) -> Option<Token> {
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

        token
    }

    pub const fn line(&self) -> usize {
        self.reader.line()
    }

    pub const fn col(&self) -> usize {
        self.reader.col()
    }

    fn read_text(&mut self) -> Option<Token> {
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

    fn read_param(&mut self) -> Option<Token> {
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
                Token::Identifier(String::from(text))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_index() {
        let mut lexer = Lexer::new("$1");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_index_text() {
        let mut lexer = Lexer::new("pre $1 post");

        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("pre "))));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from(" post"))));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_named() {
        let mut lexer = Lexer::new("$VAR");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_named_text() {
        let mut lexer = Lexer::new("pre $VAR post");

        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("pre "))));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from(" post"))));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_index() {
        let mut lexer = Lexer::new("${1}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_index_text() {
        let mut lexer = Lexer::new("pre ${1} post");

        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("pre "))));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from(" post"))));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_named() {
        let mut lexer = Lexer::new("${VAR}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_named_text() {
        let mut lexer = Lexer::new("pre ${VAR} post");

        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("pre "))));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from(" post"))));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_index() {
        let mut lexer = Lexer::new("${1-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("default")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_named() {
        let mut lexer = Lexer::new("${VAR-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("default")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_pattern() {
        let mut lexer = Lexer::new("${VAR-$DEF}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("DEF")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_index_no_empty() {
        let mut lexer = Lexer::new("${1:-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("default")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("default")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_pattern_no_empty() {
        let mut lexer = Lexer::new("${VAR:-$DEF}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("DEF")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_index() {
        let mut lexer = Lexer::new("${1+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("alt"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_named() {
        let mut lexer = Lexer::new("${VAR+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("alt"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_pattern() {
        let mut lexer = Lexer::new("${VAR+$ALT}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("ALT")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_index_no_empty() {
        let mut lexer = Lexer::new("${1:+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("alt"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("alt"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_pattern_no_empty() {
        let mut lexer = Lexer::new("${VAR:+$ALT}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("ALT")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_index() {
        let mut lexer = Lexer::new("${1?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("msg"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_named() {
        let mut lexer = Lexer::new("${VAR?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("msg"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_index_no_empty() {
        let mut lexer = Lexer::new("${1:?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("msg"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("msg"))));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_no_message() {
        let mut lexer = Lexer::new("${VAR?}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_no_message_no_empty() {
        let mut lexer = Lexer::new("${VAR:?}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn len_index() {
        let mut lexer = Lexer::new("${#1}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::PoundSign));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn len_named() {
        let mut lexer = Lexer::new("${#VAR}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::PoundSign));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn arity() {
        let mut lexer = Lexer::new("${#}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::PoundSign));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn ref_index() {
        let mut lexer = Lexer::new("${!1}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::ExclamationMark));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn ref_named() {
        let mut lexer = Lexer::new("${!VAR}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::ExclamationMark));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_escaped() {
        let mut lexer = Lexer::new("pre $${VAR post");

        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("pre ${VAR post")))
        );
        assert_eq!(lexer.next_token(), None);

        let mut lexer = Lexer::new("$$VAR$$");

        assert_eq!(lexer.next_token(), Some(Token::Text(String::from("$VAR$"))));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn pattern_escaped() {
        let mut lexer = Lexer::new("${VAR:-$$woop$}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from("$woop$")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn multiline() {
        let mut lexer = Lexer::new("$1 woop\n${VAR}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Text(String::from(" woop\n")))
        );
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(String::from("VAR")))
        );
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }
}
