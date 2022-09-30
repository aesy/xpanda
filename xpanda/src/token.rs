use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    /// Any text outside of a param
    Text(Cow<'a, str>),
    /// The name of a named variable or environment variable
    Identifier(Cow<'a, str>),
    /// The index of a positional variable
    Index(usize),
    OpenBrace,
    CloseBrace,
    DollarSign,
    Colon,
    Dash,
    Plus,
    QuestionMark,
    PoundSign,
    ExclamationMark,
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
            Token::ExclamationMark => write!(f, "'!'"),
        }
    }
}
