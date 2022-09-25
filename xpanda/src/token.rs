use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    Text(&'a str),
    Identifier(&'a str),
    Index(usize),
    OpenBrace,
    CloseBrace,
    DollarSign,
    Colon,
    Dash,
    Plus,
    QuestionMark,
    PoundSign,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Text(text) => write!(f, "\"{}\"", text),
            Token::Identifier(name) => write!(f, "\"{}\"", name),
            Token::Index(index) => write!(f, "{}", index),
            Token::OpenBrace => write!(f, "'{{'"),
            Token::CloseBrace => write!(f, "'}}'"),
            Token::DollarSign => write!(f, "'$'"),
            Token::Colon => write!(f, "':'"),
            Token::Dash => write!(f, "'-'"),
            Token::Plus => write!(f, "'+'"),
            Token::QuestionMark => write!(f, "'?'"),
            Token::PoundSign => write!(f, "'#'"),
        }
    }
}
