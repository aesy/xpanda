use std::borrow::Cow;
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
            Self::Named(name) => write!(f, "{name}"),
            Self::Indexed(index) => write!(f, "{index}"),
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
        default: Vec<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    // ${identifier+alt} | ${identifier:+alt}
    WithAlt {
        identifier: Identifier<'a>,
        alt: Vec<Node<'a>>,
        treat_empty_as_unset: bool,
    },
    // ${identifier?} | ${identifier:?} | ${identifier?error} | ${identifier:?error}
    WithError {
        identifier: Identifier<'a>,
        error: Vec<Node<'a>>,
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
    // ${identifier#pattern} | ${identifier##pattern}
    PrefixRemoval {
        identifier: Identifier<'a>,
        pattern: Vec<Node<'a>>,
        greedy: bool,
    },
    // ${identifier%pattern} | ${identifier%%pattern}
    SuffixRemoval {
        identifier: Identifier<'a>,
        pattern: Vec<Node<'a>>,
        greedy: bool,
    },
    // ${identifier/pattern} | ${identifier//pattern}
    // ${identifier/pattern/replacement} | ${identifier//pattern/replacement}
    Replace {
        identifier: Identifier<'a>,
        pattern: Vec<Node<'a>>,
        replacement: Vec<Node<'a>>,
        all_occurrences: bool,
    },
    // ${identifier:offset} | ${identifier:offset:length}
    Sub {
        identifier: Identifier<'a>,
        offset: isize,
        length: Option<isize>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node<'a> {
    Text { value: Cow<'a, str>, literal: bool },
    Param(Param<'a>),
}

impl<'a> Node<'a> {
    pub fn text(value: impl Into<Cow<'a, str>>) -> Self {
        Self::Text {
            value: value.into(),
            literal: false,
        }
    }

    pub fn literal(value: impl Into<Cow<'a, str>>) -> Self {
        Self::Text {
            value: value.into(),
            literal: true,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ast<'a> {
    pub nodes: Vec<Node<'a>>,
}

impl<'a> Ast<'a> {
    pub const fn new(nodes: Vec<Node<'a>>) -> Self {
        Self { nodes }
    }
}
