use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    /// Any text outside a param, or a run of non-operator characters inside a
    /// param body. Borrowed straight from the input when possible (no `$$`
    /// escape, no concatenation).
    Text(Cow<'a, str>),
    /// A region that originated from a quoted span (`"..."` or `'...'`).
    /// The quote characters themselves are not included.
    QuotedText(Cow<'a, str>),
    /// The name of a named variable or environment variable
    Identifier(&'a str),
    /// The index of a positional variable
    Index(usize),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    DollarSign,
    Colon,
    Dash,
    Plus,
    QuestionMark,
    PoundSign,
    ExclamationMark,
    ForwardSlash,
    Percent,
    Comma,
    Caret,
    Tilde,
}

impl Token<'_> {
    pub fn as_body_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) | Self::QuotedText(s) => Some(s),
            Self::Colon => Some(":"),
            Self::Dash => Some("-"),
            Self::Plus => Some("+"),
            Self::QuestionMark => Some("?"),
            Self::PoundSign => Some("#"),
            Self::Percent => Some("%"),
            Self::Comma => Some(","),
            Self::Caret => Some("^"),
            Self::Tilde => Some("~"),
            Self::OpenParen => Some("("),
            Self::CloseParen => Some(")"),
            Self::ExclamationMark => Some("!"),
            Self::ForwardSlash => Some("/"),
            _ => None,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Text(text) | Self::QuotedText(text) => write!(f, "\"{text}\""),
            Self::Identifier(name) => write!(f, "\"{name}\""),
            Self::Index(index) => write!(f, "{index}"),
            Self::OpenParen => write!(f, "'('"),
            Self::CloseParen => write!(f, "')'"),
            Self::OpenBrace => write!(f, "'{{'"),
            Self::CloseBrace => write!(f, "'}}'"),
            Self::DollarSign => write!(f, "'$'"),
            Self::Colon => write!(f, "':'"),
            Self::Dash => write!(f, "'-'"),
            Self::Plus => write!(f, "'+'"),
            Self::QuestionMark => write!(f, "'?'"),
            Self::PoundSign => write!(f, "'#'"),
            Self::ExclamationMark => write!(f, "'!'"),
            Self::ForwardSlash => write!(f, "'/'"),
            Self::Percent => write!(f, "'%'"),
            Self::Comma => write!(f, "','"),
            Self::Caret => write!(f, "'^'"),
            Self::Tilde => write!(f, "'~'"),
        }
    }
}
