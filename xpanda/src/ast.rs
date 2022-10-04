use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Identifier {
    // $name
    Named(String),
    // $1
    Indexed(usize),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{}", name),
            Self::Indexed(index) => write!(f, "{}", index),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Param {
    // $identifier | ${identifier}
    Simple {
        identifier: Identifier,
    },
    // ${identifier-default} | ${identifier:-default}
    WithDefault {
        identifier: Identifier,
        default: Box<Node>,
        treat_empty_as_unset: bool,
    },
    // ${identifier+default} | ${identifier:+default}
    WithAlt {
        identifier: Identifier,
        alt: Box<Node>,
        treat_empty_as_unset: bool,
    },
    // ${identifier?} | ${identifier:?} | ${identifier?error} | ${identifier:?error}
    WithError {
        identifier: Identifier,
        error: Option<String>,
        treat_empty_as_unset: bool,
    },
    // ${#identifier}
    Length {
        identifier: Identifier,
    },
    // ${#}
    Arity,
    // ${!identifier}
    Ref {
        identifier: Identifier,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Text(String),
    Param(Param),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ast {
    pub nodes: Vec<Node>,
}

impl Ast {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }
}
