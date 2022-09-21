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

        assert_eq!(lexer.next_token(), Some(Token::Text("pre ")));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Text(" post")));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_named() {
        let mut lexer = Lexer::new("$VAR");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple_named_text() {
        let mut lexer = Lexer::new("pre $VAR post");

        assert_eq!(lexer.next_token(), Some(Token::Text("pre ")));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Text(" post")));
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

        assert_eq!(lexer.next_token(), Some(Token::Text("pre ")));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), Some(Token::Text(" post")));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_named() {
        let mut lexer = Lexer::new("${VAR}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn braced_named_text() {
        let mut lexer = Lexer::new("pre ${VAR} post");

        assert_eq!(lexer.next_token(), Some(Token::Text("pre ")));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), Some(Token::Text(" post")));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_index() {
        let mut lexer = Lexer::new("${1-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Index(1)));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::Text("default")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_named() {
        let mut lexer = Lexer::new("${VAR-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::Text("default")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_pattern() {
        let mut lexer = Lexer::new("${VAR-$DEF}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("DEF")));
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
        assert_eq!(lexer.next_token(), Some(Token::Text("default")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:-default}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::Text("default")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn default_pattern_no_empty() {
        let mut lexer = Lexer::new("${VAR:-$DEF}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Dash));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("DEF")));
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
        assert_eq!(lexer.next_token(), Some(Token::Text("alt")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_named() {
        let mut lexer = Lexer::new("${VAR+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text("alt")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_pattern() {
        let mut lexer = Lexer::new("${VAR+$ALT}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("ALT")));
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
        assert_eq!(lexer.next_token(), Some(Token::Text("alt")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:+alt}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Text("alt")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn alt_pattern_no_empty() {
        let mut lexer = Lexer::new("${VAR:+$ALT}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("ALT")));
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
        assert_eq!(lexer.next_token(), Some(Token::Text("msg")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_named() {
        let mut lexer = Lexer::new("${VAR?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text("msg")));
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
        assert_eq!(lexer.next_token(), Some(Token::Text("msg")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_named_no_empty() {
        let mut lexer = Lexer::new("${VAR:?msg}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::Colon));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::Text("msg")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_no_message() {
        let mut lexer = Lexer::new("${VAR?}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::QuestionMark));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn error_no_message_no_empty() {
        let mut lexer = Lexer::new("${VAR:?}");

        assert_eq!(lexer.next_token(), Some(Token::DollarSign));
        assert_eq!(lexer.next_token(), Some(Token::OpenBrace));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
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
        assert_eq!(lexer.next_token(), Some(Token::Identifier("VAR")));
        assert_eq!(lexer.next_token(), Some(Token::CloseBrace));
        assert_eq!(lexer.next_token(), None);
    }
}
