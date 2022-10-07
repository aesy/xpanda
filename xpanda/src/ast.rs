use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Identifier<'a> {
    // $name
    Named(&'a str),
    // $1
    Indexed(usize),
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{}", name),
            Self::Indexed(index) => write!(f, "{}", index),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Modifier {
    // ${identifier^} | ${identifier^^}
    Upper { all: bool },
    // ${identifier,} | ${identifier,,}
    Lower { all: bool },
    // ${identifier~} | ${identifier~~}
    Reverse { all: bool },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Param<'a> {
    // $identifier | ${identifier}
    // ${identifier^} | ${identifier^^}
    // ${identifier,} | ${identifier,,}
    // ${identifier~} | ${identifier~~}
    Simple {
        identifier: Identifier<'a>,
        modifier: Option<Modifier>,
    },
    // ${identifier-default} | ${identifier:-default}
    WithDefault {
        identifier: Identifier<'a>,
        default: Box<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    // ${identifier+default} | ${identifier:+default}
    WithAlt {
        identifier: Identifier<'a>,
        alt: Box<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    // ${identifier?} | ${identifier:?} | ${identifier?error} | ${identifier:?error}
    WithError {
        identifier: Identifier<'a>,
        error: Option<String>,
        treat_empty_as_unset: bool,
    },
    // ${#identifier}
    Length {
        identifier: Identifier<'a>,
    },
    // ${#}
    Arity,
    // ${!identifier}
    Ref {
        identifier: Identifier<'a>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node<'a> {
    Text(String),
    Param(Param<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ast<'a> {
    pub nodes: Vec<Node<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new(nodes: Vec<Node<'a>>) -> Self {
        Self { nodes }
    }
}
