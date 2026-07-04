use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::forward_peekable::{ForwardPeekable, IteratorExt};
use crate::lexer::{self, Lexer};
use crate::position::Position;
use crate::token::Token;
use std::borrow::Cow;
use std::mem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub position: Position,
}

impl Error {
    const fn new(message: String, position: Position) -> Self {
        Self { message, position }
    }
}

pub struct Parser<'a> {
    iter: ForwardPeekable<lexer::IterMut<'a>>,
    position: Option<Position>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            iter: lexer.into_iter().forward_peekable(),
            position: None,
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
        self.iter.peek().map(|(token, _)| token)
    }

    #[must_use]
    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some((token, position)) = self.iter.next() {
            self.position = Some(position);
            Some(token)
        } else {
            None
        }
    }

    fn skip_token(&mut self) {
        self.next_token();
    }

    fn expect_token(&mut self, expected: &Token<'a>) -> Result<(), Error> {
        match self.next_token() {
            Some(ref actual) if actual == expected => Ok(()),
            Some(unexpected) => {
                Err(self.create_error(format!("Expected {expected}, found {unexpected}")))
            },
            None => Err(self.create_error(format!("Expected {expected}, found EOF"))),
        }
    }

    fn parse_node(&mut self) -> Result<Node<'a>, Error> {
        match self.peek_token() {
            Some(Token::Text(_)) => {
                let text = self.parse_text()?.unwrap_or_default();

                Ok(Node::text(text))
            },
            Some(Token::DollarSign) => {
                self.skip_token();

                Ok(Node::Param(self.parse_param()?))
            },
            Some(token) => {
                let msg = format!("Unexpected token {token}");

                Err(self.create_error(msg))
            },
            None => Err(self.create_error("Unexpected EOF")),
        }
    }

    fn parse_param(&mut self) -> Result<Param<'a>, Error> {
        match self.peek_token() {
            Some(Token::OpenBrace) => {
                self.skip_token();

                let param = match self.peek_token() {
                    Some(Token::PoundSign) => self.parse_len_or_arity_param(),
                    Some(Token::ExclamationMark) => self.parse_ref_param(),
                    Some(_) => {
                        let identifier = self.parse_identifier()?;

                        match self.peek_token() {
                            Some(Token::PoundSign) => self.parse_trim_prefix_param(identifier),
                            Some(Token::Percent) => self.parse_trim_suffix_param(identifier),
                            Some(Token::ForwardSlash) => self.parse_replace_param(identifier),
                            Some(Token::Caret) => self.parse_uppercase_param(identifier),
                            Some(Token::Comma) => self.parse_lowercase_param(identifier),
                            Some(Token::Tilde) => self.parse_reverse_case_param(identifier),
                            Some(_) => self.parse_default_alt_error_or_sub_param(identifier),
                            _ => Err(self.create_error("Invalid param, unexpected EOF")),
                        }
                    },
                    None => Err(self.create_error("Expected param, found EOF")),
                }?;

                self.expect_token(&Token::CloseBrace)?;

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
            None => Err(self.create_error("Expected identifier or close brace, found EOF")),
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
            let offset = self.parse_number_maybe_parens()?;
            let length = if self.peek_token() == Some(&Token::Colon) {
                self.skip_token();
                Some(self.parse_number_maybe_parens()?)
            } else {
                None
            };

            return Ok(Param::Sub {
                identifier,
                offset,
                length,
            });
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
                let offset = self.parse_number_maybe_parens()?;
                let length = if self.peek_token() == Some(&Token::Colon) {
                    self.skip_token();
                    Some(self.parse_number_maybe_parens()?)
                } else {
                    None
                };

                Ok(Param::Sub {
                    identifier,
                    offset,
                    length,
                })
            },
            Some(token) => {
                let msg = format!("Invalid param, unexpected token {token}");
                Err(self.create_error(msg))
            },
            None => Err(self.create_error("Invalid param, unexpected EOF")),
        }
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

    fn parse_uppercase_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::Caret)?;

        let all = if self.peek_token() == Some(&Token::Caret) {
            self.skip_token();
            true
        } else {
            false
        };

        Ok(Param::Simple {
            identifier,
            modifier: Some(Modifier::Upper { all }),
        })
    }

    fn parse_lowercase_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::Comma)?;

        let all = if self.peek_token() == Some(&Token::Comma) {
            self.skip_token();
            true
        } else {
            false
        };

        Ok(Param::Simple {
            identifier,
            modifier: Some(Modifier::Lower { all }),
        })
    }

    fn parse_reverse_case_param(&mut self, identifier: Identifier<'a>) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::Tilde)?;

        let all = if self.peek_token() == Some(&Token::Tilde) {
            self.skip_token();
            true
        } else {
            false
        };

        Ok(Param::Simple {
            identifier,
            modifier: Some(Modifier::Reverse { all }),
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
            Some(token) => Err(self.create_error(format!("Expected text, found {token}"))),
            None => Ok(None),
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, Error> {
        match self.next_token() {
            Some(Token::Identifier(name)) => Ok(Identifier::Named(name)),
            Some(Token::Index(index)) => Ok(Identifier::Indexed(index)),
            Some(token) => Err(self.create_error(format!("Expected identifier, found {token}"))),
            None => Err(self.create_error("Expected identifier, found EOF")),
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
                    .map_err(|_| self.create_error(format!("Invalid number: \"{buf}\"")))
            },
            Some(token) => Err(self.create_error(format!("Expected number, found {token}"))),
            None => Err(self.create_error("Expected number, found EOF")),
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
                    return Err(self.create_error("Unexpected EOF"));
                },
                Some(t) if is_terminator(t) => {
                    break;
                },
                Some(Token::DollarSign) => {
                    flush_buffers(&mut nodes, &mut text_buf, &mut literal_buf);
                    self.skip_token();
                    nodes.push(Node::Param(self.parse_param()?));
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

    fn create_error(&mut self, msg: impl Into<String>) -> Error {
        Error::new(msg.into(), self.position.take().unwrap_or_default())
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
