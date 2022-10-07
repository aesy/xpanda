use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    /// Any text outside of a param
    Text(String),
    /// The name of a named variable or environment variable
    Identifier(&'a str),
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "\"{}\"", text),
            Self::Identifier(name) => write!(f, "\"{}\"", name),
            Self::Index(index) => write!(f, "{}", index),
            Self::OpenBrace => write!(f, "'{{'"),
            Self::CloseBrace => write!(f, "'}}'"),
            Self::DollarSign => write!(f, "'$'"),
            Self::Colon => write!(f, "':'"),
            Self::Dash => write!(f, "'-'"),
            Self::Plus => write!(f, "'+'"),
            Self::QuestionMark => write!(f, "'?'"),
            Self::PoundSign => write!(f, "'#'"),
            Self::ExclamationMark => write!(f, "'!'"),
        }
    }
}
