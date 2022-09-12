#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Identifier<'a> {
    Named(&'a str),
    Indexed(usize),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node<'a> {
    Text(&'a str),
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
