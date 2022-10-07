use crate::ast::{Ast, Identifier, Node, Param};
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
    pub const fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, Error> {
        let mut nodes = Vec::new();

        while self.peek_token().is_some() {
            let node = self.parse_node()?;
            nodes.push(node);
        }

        Ok(Ast::new(nodes))
    }

    #[must_use]
    fn peek_token(&mut self) -> Option<&Token<'a>> {
        self.peeked
            .get_or_insert_with(|| self.lexer.next_token())
            .as_ref()
    }

    #[must_use]
    fn next_token(&mut self) -> Option<Token<'a>> {
        match self.peeked.take() {
            Some(option) => option,
            None => self.lexer.next_token(),
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

                match self.peek_token() {
                    Some(Token::PoundSign) => self.parse_len_or_arity_param(),
                    Some(Token::ExclamationMark) => self.parse_ref_param(),
                    Some(_) => self.parse_other_param(),
                    None => Err(self.create_error("Expected param, found EOF")),
                }
            },
            _ => self.parse_simple_param(),
        }
    }

    fn parse_len_or_arity_param(&mut self) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::ExclamationMark)?;

        match self.peek_token() {
            Some(Token::CloseBrace) => {
                self.skip_token();
                Ok(Param::Arity)
            },
            Some(_) => {
                let identifier = self.parse_identifier()?;
                self.expect_token(&Token::CloseBrace)?;
                Ok(Param::Length { identifier })
            },
            _ => Err(self.create_error("Expected identifier or close brace, found EOF")),
        }
    }

    fn parse_ref_param(&mut self) -> Result<Param<'a>, Error> {
        self.expect_token(&Token::ExclamationMark)?;
        let identifier = self.parse_identifier()?;
        self.expect_token(&Token::CloseBrace)?;
        Ok(Param::Ref { identifier })
    }

    fn parse_other_param(&mut self) -> Result<Param, Error> {
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
                error: match self.peek_token() {
                    Some(Token::Text(_)) => self.parse_text()?.map(String::from),
                    _ => None,
                },
                treat_empty_as_unset,
            },
            Some(Token::CloseBrace) => Param::Simple { identifier },
            Some(token) => {
                return Err(self.create_error(format!("Invalid param, unexpected token {}", token)))
            },
            _ => return Err(self.create_error("Invalid param, unexpected EOF")),
        };

        if token != Some(Token::CloseBrace) {
            self.expect_token(&Token::CloseBrace)?;
        }

        Ok(param)
    }

    fn parse_simple_param(&mut self) -> Result<Param<'a>, Error> {
        let identifier = self.parse_identifier()?;
        Ok(Param::Simple { identifier })
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

    fn create_error(&self, msg: impl Into<String>) -> Error {
        Error::new(msg.into(), self.lexer.line(), self.lexer.col())
    }
}
