use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::forward_peekable::{ForwardPeekable, IteratorExt};
use crate::lexer::{self, Lexer};
use crate::position::Position;
use crate::token::Token;

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
            Some(expected) => Ok(()),
            Some(unexpected) => {
                Err(self.create_error(format!("Expected {}, found {}", expected, unexpected)))
            },
            _ => Err(self.create_error(format!("Expected {}, found EOF", expected))),
        }
    }

    fn parse_node(&mut self) -> Result<Node<'a>, Error> {
        match self.peek_token() {
            Some(Token::Text(_)) => Ok(Node::Text(
                self.parse_text()?.unwrap_or_else(|| String::from("")),
            )),
            Some(Token::DollarSign) => {
                self.skip_token();
                Ok(Node::Param(self.parse_param()?))
            },
            Some(token) => {
                let msg = format!("Unexpected token {}", token);
                Err(self.create_error(msg))
            },
            _ => Err(self.create_error("Unexpected EOF")),
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
        self.expect_token(&Token::ExclamationMark)?;

        match self.peek_token() {
            Some(Token::CloseBrace) => Ok(Param::Arity),
            Some(_) => Ok(Param::Length {
                identifier: self.parse_identifier()?,
            }),
            _ => Err(self.create_error("Expected identifier or close brace, found EOF")),
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

        match self.peek_token() {
            // TODO Sub if is integer or paren
            Some(Token::Dash) => {
                self.skip_token();

                Ok(Param::WithDefault {
                    identifier,
                    default: Box::new(self.parse_node()?),
                    treat_empty_as_unset,
                })
            },
            Some(Token::Plus) => {
                self.skip_token();

                Ok(Param::WithAlt {
                    identifier,
                    alt: Box::new(self.parse_node()?),
                    treat_empty_as_unset,
                })
            },
            Some(Token::QuestionMark) => {
                self.skip_token();

                Ok(Param::WithError {
                    identifier,
                    error: match self.peek_token() {
                        Some(Token::Text(_)) => self.parse_text()?,
                        _ => None,
                    },
                    treat_empty_as_unset,
                })
            },
            Some(Token::CloseBrace) => Ok(Param::Simple {
                identifier,
                modifier: None,
            }),
            Some(token) => {
                let msg = format!("Invalid param, unexpected token {}", token);
                Err(self.create_error(msg))
            },
            _ => Err(self.create_error("Invalid param, unexpected EOF")),
        }
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

    fn parse_text(&mut self) -> Result<Option<String>, Error> {
        match self.next_token() {
            Some(Token::Text(text)) => Ok(Some(text)),
            Some(token) => Err(self.create_error(format!("Expected text, found {}", token))),
            None => Ok(None),
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, Error> {
        match self.next_token() {
            Some(Token::Identifier(name)) => Ok(Identifier::Named(name)),
            Some(Token::Index(index)) => Ok(Identifier::Indexed(index)),
            Some(token) => Err(self.create_error(format!("Expected identifier, found {}", token))),
            None => Err(self.create_error("Expected identifier, found EOF")),
        }
    }

    fn create_error(&mut self, msg: impl Into<String>) -> Error {
        Error::new(msg.into(), self.position.take().unwrap_or_default())
    }
}
