use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::error::ErrorKind;
use crate::forward_peekable::{ForwardPeekable, IteratorExt};
use crate::lexer::{self, Lexer};
use crate::position::Position;
use crate::token::Token;
use std::borrow::Cow;
use std::fmt::Display;
use std::mem;

// Parsing and evaluation recurse per level, and a stack overflow can't be
// caught by the caller.
const MAX_NESTING_DEPTH: usize = 256;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position,
}

impl Error {
    const fn new(kind: ErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

pub struct Parser<'a> {
    iter: ForwardPeekable<lexer::IterMut<'a>>,
    position: Option<Position>,
    end_position: Position,
    depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            iter: lexer.into_iter().forward_peekable(),
            position: None,
            end_position: Position::default(),
            depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast<'a>, Error> {
        let mut nodes = Vec::new();

        while self.peek_token().is_some() {
            let node = self.parse_node()?;
            nodes.push(node);
        }

        Ok(Ast::new(nodes))
    }

    #[must_use]
    fn peek_token(&mut self) -> Option<&Token<'a>> {
        if self.iter.peek().is_none() {
            self.position = Some(self.end_position.clone());
        }

        self.iter.peek().map(|(token, ..)| token)
    }

    #[must_use]
    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some((token, start, end)) = self.iter.next() {
            self.position = Some(start);
            self.end_position = end;
            Some(token)
        } else {
            self.position = Some(self.end_position.clone());
            None
        }
    }

    fn skip_token(&mut self) {
        let _ = self.next_token();
    }

    fn expect_token(&mut self, expected: &Token<'a>) -> Result<(), Error> {
        match self.next_token() {
            Some(ref actual) if actual == expected => Ok(()),
            Some(unexpected) => Err(self.expected_error(expected, &unexpected)),
            None => Err(self.expected_error(expected, &"EOF")),
        }
    }

    fn parse_node(&mut self) -> Result<Node<'a>, Error> {
        match self.peek_token() {
            Some(Token::Text(_)) => {
                let text = self.parse_text()?.unwrap_or_default();

                Ok(Node::text(text))
            },
            Some(Token::DollarSign) => self.parse_param_node(),
            Some(token) => {
                let kind = ErrorKind::UnexpectedToken {
                    token: token.to_string(),
                };
                self.skip_token();

                Err(self.create_error(kind))
            },
            None => Err(self.create_error(ErrorKind::UnexpectedEof)),
        }
    }

    fn parse_param_node(&mut self) -> Result<Node<'a>, Error> {
        self.skip_token();

        let position = self.position.clone().unwrap_or_default();
        let param = self.parse_param()?;

        Ok(Node::Param { param, position })
    }

    fn parse_param(&mut self) -> Result<Param<'a>, Error> {
        match self.peek_token() {
            Some(Token::OpenBrace) => {
                self.skip_token();

                if self.depth == MAX_NESTING_DEPTH {
                    return Err(self.create_error(ErrorKind::NestingTooDeep {
                        max_depth: MAX_NESTING_DEPTH,
                    }));
                }

                self.depth += 1;

                let param = match self.peek_token() {
                    Some(Token::PoundSign) => self.parse_len_or_arity_param(),
                    Some(Token::ExclamationMark) => self.parse_ref_param(),
                    Some(_) => {
                        let identifier = self.parse_identifier()?;

                        match self.peek_token() {
                            Some(Token::PoundSign) => self.parse_trim_prefix_param(identifier),
                            Some(Token::Percent) => self.parse_trim_suffix_param(identifier),
                            Some(Token::ForwardSlash) => self.parse_replace_param(identifier),
                            Some(Token::Caret) => {
                                self.parse_case_param(identifier, &Token::Caret, |all| {
                                    Modifier::Upper { all }
                                })
                            },
                            Some(Token::Comma) => {
                                self.parse_case_param(identifier, &Token::Comma, |all| {
                                    Modifier::Lower { all }
                                })
                            },
                            Some(Token::Tilde) => {
                                self.parse_case_param(identifier, &Token::Tilde, |all| {
                                    Modifier::Reverse { all }
                                })
                            },
                            Some(_) => self.parse_default_alt_error_or_sub_param(identifier),
                            _ => Err(self.create_error(ErrorKind::InvalidParamEof)),
                        }
                    },
                    None => Err(self.expected_error(&"param", &"EOF")),
                }?;

                self.expect_token(&Token::CloseBrace)?;
                self.depth -= 1;

                Ok(param)
            },
            _ => self.parse_simple_param(),
        }
    }

    fn parse_len_or_arity_param(&mut self) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::PoundSign)?;

        match self.peek_token() {
            Some(Token::CloseBrace) => Ok(Param::Arity),
            Some(_) => Ok(Param::Length {
                identifier: self.parse_identifier()?,
            }),
            None => Err(self.expected_error(&"identifier or close brace", &"EOF")),
        }
    }

    fn parse_ref_param(&mut self) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::ExclamationMark)?;

        Ok(Param::Ref {
            identifier: self.parse_identifier()?,
        })
    }

    fn parse_default_alt_error_or_sub_param(
        &mut self,
        identifier: Identifier<'a>,
    ) -> Result<Param<'a>, Error> {
        let treat_empty_as_unset = if self.peek_token() == Some(&Token::Colon) {
            self.skip_token();
            true
        } else {
            false
        };

        let leading_ws = matches!(
            self.peek_token(),
            Some(Token::Text(t)) if t.chars().all(|c| c == ' ')
        );

        if treat_empty_as_unset && leading_ws {
            self.skip_token();

            return self.parse_sub_param(identifier);
        }

        match self.peek_token() {
            Some(Token::Dash) => {
                self.skip_token();

                Ok(Param::WithDefault {
                    identifier,
                    default: self.parse_body_to_close_brace()?,
                    treat_empty_as_unset,
                })
            },
            Some(Token::Plus) => {
                self.skip_token();

                Ok(Param::WithAlt {
                    identifier,
                    alt: self.parse_body_to_close_brace()?,
                    treat_empty_as_unset,
                })
            },
            Some(Token::QuestionMark) => {
                self.skip_token();

                Ok(Param::WithError {
                    identifier,
                    error: self.parse_body_to_close_brace()?,
                    treat_empty_as_unset,
                })
            },
            Some(Token::CloseBrace) => Ok(Param::Simple {
                identifier,
                modifier: None,
            }),
            Some(Token::Text(_) | Token::OpenParen) if treat_empty_as_unset => {
                self.parse_sub_param(identifier)
            },
            Some(token) => {
                let kind = ErrorKind::InvalidParamToken {
                    token: token.to_string(),
                };
                self.skip_token();

                Err(self.create_error(kind))
            },
            None => Err(self.create_error(ErrorKind::InvalidParamEof)),
        }
    }

    fn parse_sub_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        let offset = self.parse_number_maybe_parens()?;
        let length = if self.peek_token() == Some(&Token::Colon) {
            self.skip_token();

            // Bash treats an omitted length as 0.
            if self.peek_token() == Some(&Token::CloseBrace) {
                Some(0)
            } else {
                Some(self.parse_number_maybe_parens()?)
            }
        } else {
            None
        };

        Ok(Param::Sub {
            identifier,
            offset,
            length,
        })
    }

    fn parse_trim_prefix_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::PoundSign)?;

        let greedy = if self.peek_token() == Some(&Token::PoundSign) {
            self.skip_token();
            true
        } else {
            false
        };

        let pattern = self.parse_body_to_close_brace()?;

        Ok(Param::PrefixRemoval {
            identifier,
            pattern,
            greedy,
        })
    }

    fn parse_trim_suffix_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::Percent)?;

        let greedy = if self.peek_token() == Some(&Token::Percent) {
            self.skip_token();
            true
        } else {
            false
        };

        let pattern = self.parse_body_to_close_brace()?;

        Ok(Param::SuffixRemoval {
            identifier,
            pattern,
            greedy,
        })
    }

    fn parse_replace_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::ForwardSlash)?;

        let all_occurrences = if self.peek_token() == Some(&Token::ForwardSlash) {
            self.skip_token();
            true
        } else {
            false
        };

        let pattern =
            self.parse_body_until(|t| matches!(t, Token::ForwardSlash | Token::CloseBrace))?;

        let replacement = if self.peek_token() == Some(&Token::ForwardSlash) {
            self.skip_token();
            self.parse_body_to_close_brace()?
        } else {
            Vec::new()
        };

        Ok(Param::Replace {
            identifier,
            pattern,
            replacement,
            all_occurrences,
        })
    }

    fn parse_case_param(
        &mut self,
        identifier: Identifier<'a>,
        operator: &Token<'a>,
        modifier: fn(bool) -> Modifier,
    ) -> Result<Param<'a>, Error> {
        self.expect_token(operator)?;

        let all = if self.peek_token() == Some(operator) {
            self.skip_token();
            true
        } else {
            false
        };

        Ok(Param::Simple {
            identifier,
            modifier: Some(modifier(all)),
        })
    }

    fn parse_simple_param(&mut self) -> Result<Param<'a>, Error> {
        let identifier = self.parse_identifier()?;
        Ok(Param::Simple {
            identifier,
            modifier: None,
        })
    }

    fn parse_text(&mut self) -> Result<Option<Cow<'a, str>>, Error> {
        match self.next_token() {
            Some(Token::Text(text)) => Ok(Some(text)),
            Some(token) => Err(self.expected_error(&"text", &token)),
            None => Ok(None),
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, Error> {
        match self.next_token() {
            Some(Token::Identifier(name)) => Ok(Identifier::Named(name)),
            Some(Token::Index(index)) => Ok(Identifier::Indexed(index)),
            Some(token) => Err(self.expected_error(&"identifier", &token)),
            None => Err(self.expected_error(&"identifier", &"EOF")),
        }
    }

    fn parse_number_maybe_parens(&mut self) -> Result<isize, Error> {
        if self.peek_token() == Some(&Token::OpenParen) {
            self.skip_token();
            let n = self.parse_number()?;
            self.expect_token(&Token::CloseParen)?;
            Ok(n)
        } else {
            self.parse_number()
        }
    }

    fn parse_number(&mut self) -> Result<isize, Error> {
        while matches!(self.peek_token(), Some(Token::Text(t)) if t.chars().all(|c| c == ' ')) {
            self.skip_token();
        }

        let mut sign = String::new();

        if self.peek_token() == Some(&Token::Dash) {
            self.skip_token();
            sign.push('-');
        }

        while matches!(self.peek_token(), Some(Token::Text(t)) if t.chars().all(|c| c == ' ')) {
            self.skip_token();
        }

        match self.next_token() {
            Some(Token::Text(text)) => {
                let mut buf = sign;
                buf.push_str(text.trim());
                buf.parse::<isize>()
                    .map_err(|_| self.create_error(ErrorKind::InvalidNumber { value: buf.clone() }))
            },
            Some(token) => Err(self.expected_error(&"number", &token)),
            None => Err(self.expected_error(&"number", &"EOF")),
        }
    }

    fn parse_body_to_close_brace(&mut self) -> Result<Vec<Node<'a>>, Error> {
        self.parse_body_until(|t| matches!(t, Token::CloseBrace))
    }

    fn parse_body_until<F>(&mut self, is_terminator: F) -> Result<Vec<Node<'a>>, Error>
    where
        F: Fn(&Token<'a>) -> bool,
    {
        let mut nodes: Vec<Node<'a>> = Vec::new();
        let mut text_buf = String::new();
        let mut literal_buf = String::new();

        loop {
            let peek = self.peek_token();

            match peek {
                None => {
                    return Err(self.create_error(ErrorKind::UnexpectedEof));
                },
                Some(t) if is_terminator(t) => {
                    break;
                },
                Some(Token::DollarSign) => {
                    flush_buffers(&mut nodes, &mut text_buf, &mut literal_buf);
                    nodes.push(self.parse_param_node()?);
                },
                Some(Token::QuotedText(_)) => {
                    if !text_buf.is_empty() {
                        nodes.push(Node::text(mem::take(&mut text_buf)));
                    }

                    if let Some(Token::QuotedText(s)) = self.next_token() {
                        literal_buf.push_str(&s);
                    }
                },
                Some(_) => {
                    if !literal_buf.is_empty() {
                        nodes.push(Node::literal(mem::take(&mut literal_buf)));
                    }

                    let tok = self.next_token().unwrap();

                    if let Some(s) = tok.as_body_text() {
                        text_buf.push_str(s);
                    }
                },
            }
        }

        flush_buffers(&mut nodes, &mut text_buf, &mut literal_buf);

        Ok(nodes)
    }

    fn expected_error(&mut self, expected: &impl Display, found: &impl Display) -> Error {
        self.create_error(ErrorKind::Expected {
            expected: expected.to_string(),
            found: found.to_string(),
        })
    }

    fn create_error(&mut self, kind: ErrorKind) -> Error {
        Error::new(kind, self.position.take().unwrap_or_default())
    }
}

fn flush_buffers(nodes: &mut Vec<Node<'_>>, text_buf: &mut String, literal_buf: &mut String) {
    if !text_buf.is_empty() {
        nodes.push(Node::text(mem::take(text_buf)));
    }

    if !literal_buf.is_empty() {
        nodes.push(Node::literal(mem::take(literal_buf)));
    }
}
