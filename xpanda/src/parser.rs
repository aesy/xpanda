use crate::ast::{Ast, Identifier, Node, Param};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    const fn new(message: String) -> Self {
        Self { message }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Option<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> Result<Ast<'a>, ParseError> {
        let mut nodes = Vec::new();

        while self.peek_token().is_some() {
            let node = self.parse_node()?;
            nodes.push(node);
        }

        Ok(Ast::new(nodes))
    }

    fn peek_token(&mut self) -> Option<&Token<'a>> {
        self.peeked
            .get_or_insert_with(|| self.lexer.next_token())
            .as_ref()
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        match self.peeked.take() {
            Some(option) => option,
            None => self.lexer.next_token(),
        }
    }

    fn parse_node(&mut self) -> Result<Node<'a>, ParseError> {
        match self.peek_token() {
            Some(Token::Text(_)) => Ok(Node::Text(self.parse_text()?.unwrap_or(""))),
            Some(Token::DollarSign) => {
                self.next_token();
                Ok(Node::Param(self.parse_param()?))
            },
            Some(token) => Err(ParseError::new(format!("Unexpected token {:?}", token))),
            _ => Err(ParseError::new(String::from("Unexpected EOF"))),
        }
    }

    fn parse_param(&mut self) -> Result<Param<'a>, ParseError> {
        match self.peek_token() {
            Some(Token::OpenBrace) => {
                self.next_token();

                match self.peek_token() {
                    Some(_) => {
                        let identifier = self.parse_identifier()?;
                        let mut treat_empty_as_unset = false;
                        let mut token = self.next_token();

                        if token == Some(Token::Colon) {
                            treat_empty_as_unset = true;
                            token = self.next_token();
                        }

                        let param = match token {
                            Some(Token::Dash) => Param::WithDefault {
                                identifier,
                                default: Box::new(self.parse_node()?),
                                treat_empty_as_unset,
                            },
                            Some(Token::Plus) => Param::WithAlt {
                                identifier,
                                alt: Box::new(self.parse_node()?),
                                treat_empty_as_unset,
                            },
                            Some(Token::QuestionMark) => Param::WithError {
                                identifier,
                                error: self.parse_text()?,
                                treat_empty_as_unset,
                            },
                            Some(Token::CloseBrace) => Param::Simple { identifier },
                            Some(ref token) => {
                                return Err(ParseError::new(format!(
                                    "Invalid param, unexpected token {:?}",
                                    token
                                )))
                            },
                            _ => {
                                return Err(ParseError::new(String::from(
                                    "Invalid param, unexpected EOF",
                                )))
                            },
                        };

                        if token != Some(Token::CloseBrace) {
                            token = self.next_token();

                            if token != Some(Token::CloseBrace) {
                                return Err(ParseError::new(format!(
                                    "Expected close brace, found {:?}",
                                    token
                                )));
                            }
                        }

                        Ok(param)
                    },
                    None => Err(ParseError::new(String::from("Expected param, found EOF"))),
                }
            },
            Some(Token::PoundSign) => {
                self.next_token();
                Ok(Param::Length {
                    identifier: self.parse_identifier()?,
                })
            },
            _ => Ok(Param::Simple {
                identifier: self.parse_identifier()?,
            }),
        }
    }

    fn parse_text(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.next_token() {
            Some(Token::Text(text)) => Ok(Some(text)),
            Some(token) => Err(ParseError::new(format!("Expected text, found {:?}", token))),
            None => Ok(None),
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, ParseError> {
        match self.next_token() {
            Some(Token::Identifier(name)) => Ok(Identifier::Named(name)),
            Some(Token::Index(index)) => Ok(Identifier::Indexed(index)),
            Some(token) => Err(ParseError::new(format!(
                "Expected identifier, found {:?}",
                token
            ))),
            None => Err(ParseError::new(String::from(
                "Expected identifier, found EOF",
            ))),
        }
    }
}
