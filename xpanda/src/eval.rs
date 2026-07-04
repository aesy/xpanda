use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::glob::Glob;
use crate::position::Position;
use std::cmp::{max, min};
use std::collections::HashMap;

// TODO fix Position::default()

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
    pub const fn new(
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

    pub fn eval(&self, ast: &Ast) -> Result<String, Error> {
        self.eval_nodes(&ast.nodes)
    }

    fn eval_nodes(&self, nodes: &[Node]) -> Result<String, Error> {
        let mut result = String::new();
        self.append_nodes(nodes, &mut result)?;
        Ok(result)
    }

    fn append_nodes(&self, nodes: &[Node], out: &mut String) -> Result<(), Error> {
        for node in nodes {
            match node {
                Node::Text { value, .. } => out.push_str(value),
                Node::Param(param) => out.push_str(&self.eval_param(param)?),
            }
        }

        Ok(())
    }

    fn eval_nodes_as_pattern(&self, nodes: &[Node]) -> Result<String, Error> {
        let mut result = String::new();

        for node in nodes {
            match node {
                Node::Text {
                    value,
                    literal: true,
                } => {
                    for ch in value.chars() {
                        if matches!(ch, '*' | '?' | '[' | '\\') {
                            result.push('\\');
                        }

                        result.push(ch);
                    }
                },
                Node::Text {
                    value,
                    literal: false,
                } => result.push_str(value),
                Node::Param(param) => result.push_str(&self.eval_param(param)?),
            }
        }

        Ok(result)
    }

    fn eval_param(&self, param: &Param) -> Result<String, Error> {
        match param {
            Param::Simple {
                identifier,
                modifier,
            } => modifier.as_ref().map_or_else(
                || self.eval_simple_param(identifier),
                |modifier| self.eval_param_with_modifier(identifier, modifier),
            ),
            Param::WithDefault {
                identifier,
                default,
                treat_empty_as_unset,
            } => self.eval_default_param(identifier, default, *treat_empty_as_unset),
            Param::WithAlt {
                identifier,
                alt,
                treat_empty_as_unset,
            } => self.eval_alt_param(identifier, alt, *treat_empty_as_unset),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => self.eval_error_param(identifier, error, *treat_empty_as_unset),
            Param::Length { identifier } => self.eval_length_param(identifier),
            Param::Arity => self.eval_arity_param(),
            Param::Ref { identifier } => self.eval_ref_param(identifier),
            Param::PrefixRemoval {
                identifier,
                pattern,
                greedy,
            } => self.eval_prefix_removal(identifier, pattern, *greedy),
            Param::SuffixRemoval {
                identifier,
                pattern,
                greedy,
            } => self.eval_suffix_removal(identifier, pattern, *greedy),
            Param::Replace {
                identifier,
                pattern,
                replacement,
                all_occurrences,
            } => self.eval_replace(identifier, pattern, replacement, *all_occurrences),
            Param::Sub {
                identifier,
                offset,
                length,
            } => self.eval_sub_param(identifier, *offset, *length),
        }
    }

    fn eval_simple_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    Err(Error::new(
                        Self::error_message(identifier, false),
                        Position::default(),
                    ))
                } else {
                    Ok(String::new())
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

                        chars.next().map_or_else(String::new, |char| {
                            char.to_uppercase().collect::<String>() + chars.as_str()
                        })
                    }
                },
                Modifier::Lower { all } => {
                    if *all {
                        string.to_lowercase()
                    } else {
                        let mut chars = string.chars();

                        chars.next().map_or_else(String::new, |char| {
                            char.to_lowercase().collect::<String>() + chars.as_str()
                        })
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
                        chars.next().map_or_else(String::new, |char| {
                            if char.is_uppercase() {
                                char.to_lowercase().collect::<String>() + chars.as_str()
                            } else {
                                char.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        })
                    }
                },
            })
    }

    fn eval_default_param(
        &self,
        identifier: &Identifier,
        default: &[Node],
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| self.eval_nodes(default), Ok)
    }

    fn eval_alt_param(
        &self,
        identifier: &Identifier,
        alt: &[Node],
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| Ok(String::new()), |_| self.eval_nodes(alt))
    }

    fn eval_error_param(
        &self,
        identifier: &Identifier,
        error: &[Node],
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        if let Some(value) = self
            .eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
        {
            return Ok(value);
        }

        let msg = if error.is_empty() {
            Self::error_message(identifier, treat_empty_as_unset)
        } else {
            self.eval_nodes(error)?
        };

        Err(Error::new(msg, Position::default()))
    }

    fn eval_length_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
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

    fn eval_prefix_removal(
        &self,
        identifier: &Identifier,
        pattern: &[Node],
        greedy: bool,
    ) -> Result<String, Error> {
        let value = self.eval_simple_param(identifier)?;
        let pattern_str = self.eval_nodes_as_pattern(pattern)?;
        let glob = Glob::compile(&pattern_str)
            .map_err(|err| Error::new(err.message, Position::default()))?;

        Ok(glob.trim_start(&value, greedy).to_string())
    }

    fn eval_suffix_removal(
        &self,
        identifier: &Identifier,
        pattern: &[Node],
        greedy: bool,
    ) -> Result<String, Error> {
        let value = self.eval_simple_param(identifier)?;
        let pattern_str = self.eval_nodes_as_pattern(pattern)?;
        let glob = Glob::compile(&pattern_str)
            .map_err(|err| Error::new(err.message, Position::default()))?;

        Ok(glob.trim_end(&value, greedy).to_string())
    }

    fn eval_replace(
        &self,
        identifier: &Identifier,
        pattern: &[Node],
        replacement: &[Node],
        all_occurrences: bool,
    ) -> Result<String, Error> {
        let value = self.eval_simple_param(identifier)?;
        let pattern_str = self.eval_nodes_as_pattern(pattern)?;
        let glob = Glob::compile(&pattern_str)
            .map_err(|err| Error::new(err.message, Position::default()))?;
        let replacement_str = self.eval_nodes(replacement)?;

        Ok(glob.replace(&value, &replacement_str, all_occurrences))
    }

    fn eval_sub_param(
        &self,
        identifier: &Identifier,
        offset: isize,
        length: Option<isize>,
    ) -> Result<String, Error> {
        let param = self.eval_simple_param(identifier)?;
        let len = param.len();
        let start = {
            let index = if offset < 0 {
                len.saturating_sub(offset.unsigned_abs())
            } else {
                offset.unsigned_abs()
            };

            max(min(index, len), 0)
        };
        let end = length.map_or(len, |value| {
            let index = if value < 0 {
                len.saturating_sub(value.unsigned_abs())
            } else {
                start.saturating_add(value.unsigned_abs())
            };

            max(min(index, len), start)
        });

        Ok(param[start..end].to_string())
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
            format!("'{identifier}' is unset or empty")
        } else {
            format!("'{identifier}' is unset")
        }
    }
}
