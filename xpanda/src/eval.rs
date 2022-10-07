use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::parser::{self, Parser};
use crate::position::Position;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub position: Position,
}

impl Error {
    const fn new(message: String, position: Position) -> Self {
        Self { message, position }
    }
}

#[derive(Default)]
pub struct Evaluator {
    no_unset: bool,
    positional_vars: Vec<String>,
    named_vars: HashMap<String, String>,
}

impl Evaluator {
    pub fn new(
        no_unset: bool,
        positional_vars: Vec<String>,
        named_vars: HashMap<String, String>,
    ) -> Self {
        Self {
            no_unset,
            positional_vars,
            named_vars,
        }
    }

    pub fn eval(&self, ast: Ast) -> Result<String, Error> {
        let mut result = String::new();

        for node in ast.nodes {
            let text = self.eval_node(node)?;
            result.push_str(&text);
        }

        Ok(result)
    }

    fn eval_node(&self, node: Node) -> Result<String, Error> {
        match node {
            Node::Text(text) => Ok(text),
            Node::Param(param) => self.eval_param(param),
        }
    }

    fn eval_param(&self, param: Param) -> Result<String, Error> {
        match param {
            Param::Simple {
                identifier,
                modifier,
            } => modifier.map_or_else(
                || self.eval_simple_param(&identifier),
                |modifier| self.eval_param_with_modifier(&identifier, &modifier),
            ),
            Param::WithDefault {
                identifier,
                default,
                treat_empty_as_unset,
            } => self.eval_default_param(&identifier, *default, treat_empty_as_unset),
            Param::WithAlt {
                identifier,
                alt,
                treat_empty_as_unset,
            } => self.eval_alt_param(&identifier, *alt, treat_empty_as_unset),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => self.eval_error_param(&identifier, error, treat_empty_as_unset),
            Param::Length { identifier } => self.eval_length_param(&identifier),
            Param::Arity => self.eval_arity_param(),
            Param::Ref { identifier } => self.eval_ref_param(&identifier),
        }
    }

    fn eval_simple_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    // TODO wrong line/col
                    Err(Error::new(
                        Self::error_message(identifier, false),
                        Position::default(),
                    ))
                } else {
                    Ok(String::from(""))
                }
            },
            Ok,
        )
    }

    fn eval_param_with_modifier(
        &self,
        identifier: &Identifier,
        modifier: &Modifier,
    ) -> Result<String, Error> {
        self.eval_simple_param(identifier)
            .map(|string| match modifier {
                Modifier::Upper { all } => {
                    if *all {
                        string.to_uppercase()
                    } else {
                        let mut chars = string.chars();
                        match chars.next() {
                            Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
                            None => String::new(),
                        }
                    }
                },
                Modifier::Lower { all } => {
                    if *all {
                        string.to_lowercase()
                    } else {
                        let mut chars = string.chars();
                        match chars.next() {
                            Some(char) => char.to_lowercase().collect::<String>() + chars.as_str(),
                            None => String::new(),
                        }
                    }
                },
                Modifier::Reverse { all } => {
                    if *all {
                        string
                            .chars()
                            .map(|char| {
                                if char.is_uppercase() {
                                    char.to_lowercase().to_string()
                                } else {
                                    char.to_uppercase().to_string()
                                }
                            })
                            .collect()
                    } else {
                        let mut chars = string.chars();
                        match chars.next() {
                            Some(char) => {
                                if char.is_uppercase() {
                                    char.to_lowercase().collect::<String>() + chars.as_str()
                                } else {
                                    char.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            },
                            None => String::new(),
                        }
                    }
                },
            })
    }

    fn eval_default_param(
        &self,
        identifier: &Identifier,
        default: Node,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| self.eval_node(default), Ok)
    }

    fn eval_alt_param(
        &self,
        identifier: &Identifier,
        alt: Node,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| Ok(String::from("")), |_| self.eval_node(alt))
    }

    fn eval_error_param(
        &self,
        identifier: &Identifier,
        error: Option<String>,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .ok_or_else(|| {
                let msg =
                    error.unwrap_or_else(|| Self::error_message(identifier, treat_empty_as_unset));

                // TODO wrong line/col
                Error::new(msg, Position::default())
            })
    }

    fn eval_length_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    // TODO wrong line/col
                    Err(Error::new(
                        Self::error_message(identifier, false),
                        Position::default(),
                    ))
                } else {
                    Ok(String::from("0"))
                }
            },
            |value| Ok(value.len().to_string()),
        )
    }

    #[allow(clippy::unnecessary_wraps)]
    fn eval_arity_param(&self) -> Result<String, Error> {
        Ok(self.positional_vars.len().to_string())
    }

    fn eval_ref_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_simple_param(identifier)
            .and_then(|name| self.eval_simple_param(&Identifier::Named(&name)))
    }

    fn eval_identifier(&self, identifier: &Identifier) -> Option<String> {
        match identifier {
            Identifier::Named(name) => self.named_vars.get(*name).cloned(),
            Identifier::Indexed(index) => {
                if *index == 0 {
                    Some(self.positional_vars.join(" "))
                } else {
                    self.positional_vars.get(index - 1).cloned()
                }
            },
        }
    }

    fn error_message(identifier: &Identifier, treat_empty_as_unset: bool) -> String {
        if treat_empty_as_unset {
            format!("'{}' is unset or empty", identifier)
        } else {
            format!("'{}' is unset", identifier)
        }
    }
}
