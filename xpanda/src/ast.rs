use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Identifier<'a> {
    // $name
    Named(Cow<'a, str>),
    // $1
    Indexed(usize),
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Named(name) => write!(f, "{}", name),
            Identifier::Indexed(index) => write!(f, "{}", index),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Param<'a> {
    // $identifier | ${identifier}
    Simple {
        identifier: Identifier<'a>,
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
        error: Option<Cow<'a, str>>,
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
    Text(Cow<'a, str>),
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
