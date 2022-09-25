use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Identifier<'a> {
    Named(&'a str),
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
    Simple {
        identifier: Identifier<'a>,
    },
    Length {
        identifier: Identifier<'a>,
    },
    WithDefault {
        identifier: Identifier<'a>,
        default: Box<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    WithAlt {
        identifier: Identifier<'a>,
        alt: Box<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    WithError {
        identifier: Identifier<'a>,
        error: Option<&'a str>,
        treat_empty_as_unset: bool,
    },
}

impl Display for Param<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::Simple { identifier } => write!(f, "{}", identifier),
            Param::Length { identifier } => write!(f, "${{#{}}}", identifier),
            Param::WithDefault {
                identifier,
                default,
                treat_empty_as_unset,
            } => write!(
                f,
                "${{{}{}-{}}}",
                identifier,
                if *treat_empty_as_unset { ":" } else { "" },
                default
            ),
            Param::WithAlt {
                identifier,
                alt,
                treat_empty_as_unset,
            } => write!(
                f,
                "${{{}{}+{}}}",
                identifier,
                if *treat_empty_as_unset { ":" } else { "" },
                alt
            ),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => write!(
                f,
                "${{{}{}?{}}}",
                identifier,
                if *treat_empty_as_unset { ":" } else { "" },
                error.unwrap_or("")
            ),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node<'a> {
    Text(&'a str),
    Param(Param<'a>),
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Text(text) => write!(f, "{}", text),
            Node::Param(param) => write!(f, "{}", param),
        }
    }
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
