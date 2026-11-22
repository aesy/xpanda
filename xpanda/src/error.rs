use crate::position::Position;
use crate::{eval, parser};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Something specific was required, but something else was found.
    Expected { expected: String, found: String },
    /// A token that can't appear at this point in the input.
    UnexpectedToken { token: String },
    /// The input ended inside a param body.
    UnexpectedEof,
    /// A token that doesn't belong to any param form.
    InvalidParamToken { token: String },
    /// The input ended before the param was complete.
    InvalidParamEof,
    /// A substring offset or length that isn't a number.
    InvalidNumber { value: String },
    /// Params are nested deeper than the supported limit.
    NestingTooDeep { max_depth: usize },
    /// A variable is unset, in a context that requires it to be set.
    Unset { name: String },
    /// A variable is unset or empty, in a context that requires a value.
    UnsetOrEmpty { name: String },
    /// The message of a `${VAR?message}` or `${VAR:?message}` param.
    UserMessage { message: String },
    /// A glob pattern ends with an unescaped backslash.
    TrailingBackslash,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected { expected, found } => write!(f, "Expected {expected}, found {found}"),
            Self::UnexpectedToken { token } => write!(f, "Unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
            Self::InvalidParamToken { token } => {
                write!(f, "Invalid param, unexpected token {token}")
            },
            Self::InvalidParamEof => write!(f, "Invalid param, unexpected EOF"),
            Self::InvalidNumber { value } => write!(f, "Invalid number: \"{value}\""),
            Self::NestingTooDeep { max_depth } => {
                write!(f, "Params nested deeper than {max_depth} levels")
            },
            Self::Unset { name } => write!(f, "'{name}' is unset"),
            Self::UnsetOrEmpty { name } => write!(f, "'{name}' is unset or empty"),
            Self::UserMessage { message } => write!(f, "{message}"),
            Self::TrailingBackslash => write!(f, "Trailing backslash in pattern"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
    pub col: usize,
}

impl Error {
    const fn new(kind: ErrorKind, position: &Position) -> Self {
        Self {
            kind,
            line: position.line,
            col: position.col,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} {}", self.line, self.col, self.kind)
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Self::new(error.kind, &error.position)
    }
}

impl From<eval::Error> for Error {
    fn from(error: eval::Error) -> Self {
        Self::new(error.kind, &error.position.unwrap_or_default())
    }
}
