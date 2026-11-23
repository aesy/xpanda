use crate::ast::{Ast, Identifier, Modifier, Node, Param};
use crate::error::ErrorKind;
use crate::glob::{self, Glob};
use crate::position::Position;
use std::borrow::Cow;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Option<Position>,
}

impl Error {
    const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            position: None,
        }
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
                Node::Param { param, position } => {
                    out.push_str(&self.eval_param_at(param, position)?);
                },
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
                Node::Param { param, position } => {
                    result.push_str(&self.eval_param_at(param, position)?);
                },
            }
        }

        Ok(result)
    }

    fn eval_param_at(&self, param: &Param, position: &Position) -> Result<Cow<'_, str>, Error> {
        self.eval_param(param).map_err(|mut error| {
            error.position.get_or_insert_with(|| position.clone());
            error
        })
    }

    fn eval_param(&self, param: &Param) -> Result<Cow<'_, str>, Error> {
        match param {
            Param::Simple {
                identifier,
                modifier,
            } => modifier.as_ref().map_or_else(
                || self.eval_simple_param(identifier),
                |modifier| {
                    self.eval_param_with_modifier(identifier, modifier)
                        .map(Cow::Owned)
                },
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
            } => self
                .eval_alt_param(identifier, alt, *treat_empty_as_unset)
                .map(Cow::Owned),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => self.eval_error_param(identifier, error, *treat_empty_as_unset),
            Param::Length { identifier } => self.eval_length_param(identifier).map(Cow::Owned),
            Param::Arity => Ok(Cow::Owned(self.positional_vars.len().to_string())),
            Param::Ref { identifier } => self.eval_ref_param(identifier),
            Param::PrefixRemoval {
                identifier,
                pattern,
                greedy,
            } => self
                .eval_prefix_removal(identifier, pattern, *greedy)
                .map(Cow::Owned),
            Param::SuffixRemoval {
                identifier,
                pattern,
                greedy,
            } => self
                .eval_suffix_removal(identifier, pattern, *greedy)
                .map(Cow::Owned),
            Param::Replace {
                identifier,
                pattern,
                replacement,
                all_occurrences,
            } => self
                .eval_replace(identifier, pattern, replacement, *all_occurrences)
                .map(Cow::Owned),
            Param::Sub {
                identifier,
                offset,
                length,
            } => self
                .eval_sub_param(identifier, *offset, *length)
                .map(Cow::Owned),
        }
    }

    fn eval_simple_param(&self, identifier: &Identifier) -> Result<Cow<'_, str>, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    Err(Error::new(Self::unset_error(identifier, false)))
                } else {
                    Ok(Cow::Borrowed(""))
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
        let value = self.eval_simple_param(identifier)?;

        Ok(match modifier {
            Modifier::Upper { all: true } => value.to_uppercase(),
            Modifier::Upper { all: false } => {
                map_first_char(&value, |c| c.to_uppercase().to_string())
            },
            Modifier::Lower { all: true } => value.to_lowercase(),
            Modifier::Lower { all: false } => {
                map_first_char(&value, |c| c.to_lowercase().to_string())
            },
            Modifier::Reverse { all: true } => value.chars().map(reverse_case).collect(),
            Modifier::Reverse { all: false } => map_first_char(&value, reverse_case),
        })
    }

    fn eval_default_param(
        &self,
        identifier: &Identifier,
        default: &[Node],
        treat_empty_as_unset: bool,
    ) -> Result<Cow<'_, str>, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| self.eval_nodes(default).map(Cow::Owned), Ok)
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
    ) -> Result<Cow<'_, str>, Error> {
        if let Some(value) = self
            .eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
        {
            return Ok(value);
        }

        let kind = if error.is_empty() {
            Self::unset_error(identifier, treat_empty_as_unset)
        } else {
            ErrorKind::UserMessage {
                message: self.eval_nodes(error)?,
            }
        };

        Err(Error::new(kind))
    }

    fn eval_length_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    Err(Error::new(Self::unset_error(identifier, false)))
                } else {
                    Ok(String::from("0"))
                }
            },
            |value| Ok(value.chars().count().to_string()),
        )
    }

    fn eval_ref_param(&self, identifier: &Identifier) -> Result<Cow<'_, str>, Error> {
        let name = self.eval_simple_param(identifier)?;
        let is_index = !name.is_empty() && name.chars().all(|c| c.is_ascii_digit());
        let target = if is_index {
            Identifier::Indexed(name.parse().unwrap_or(usize::MAX))
        } else {
            Identifier::Named(&name)
        };

        self.eval_simple_param(&target)
    }

    fn eval_prefix_removal(
        &self,
        identifier: &Identifier,
        pattern: &[Node],
        greedy: bool,
    ) -> Result<String, Error> {
        let value = self.eval_simple_param(identifier)?;
        let glob = self.compile_pattern(pattern)?;

        Ok(glob.trim_start(&value, greedy).to_string())
    }

    fn eval_suffix_removal(
        &self,
        identifier: &Identifier,
        pattern: &[Node],
        greedy: bool,
    ) -> Result<String, Error> {
        let value = self.eval_simple_param(identifier)?;
        let glob = self.compile_pattern(pattern)?;

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
        let glob = self.compile_pattern(pattern)?;
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
        let len = param.chars().count();
        let start = if offset < 0 {
            // Bash yields nothing when a negative offset reaches past the
            // start.
            match len.checked_sub(offset.unsigned_abs()) {
                Some(index) => index,
                None => return Ok(String::new()),
            }
        } else {
            min(offset.unsigned_abs(), len)
        };
        let end = length.map_or(len, |value| {
            let index = if value < 0 {
                len.saturating_sub(value.unsigned_abs())
            } else {
                start.saturating_add(value.unsigned_abs())
            };

            max(min(index, len), start)
        });

        let byte_index = |char_index| {
            param
                .char_indices()
                .nth(char_index)
                .map_or(param.len(), |(index, _)| index)
        };

        Ok(param[byte_index(start)..byte_index(end)].to_string())
    }

    fn compile_pattern(&self, pattern: &[Node]) -> Result<Glob, Error> {
        Glob::compile(&self.eval_nodes_as_pattern(pattern)?).map_err(|error| match error {
            glob::Error::TrailingBackslash => Error::new(ErrorKind::TrailingBackslash),
        })
    }

    fn eval_identifier(&self, identifier: &Identifier) -> Option<Cow<'_, str>> {
        match identifier {
            Identifier::Named(name) => self
                .named_vars
                .get(*name)
                .map(|v| Cow::Borrowed(v.as_str())),
            Identifier::Indexed(0) => Some(Cow::Owned(self.positional_vars.join(" "))),
            Identifier::Indexed(index) => self
                .positional_vars
                .get(index - 1)
                .map(|v| Cow::Borrowed(v.as_str())),
        }
    }

    fn unset_error(identifier: &Identifier, treat_empty_as_unset: bool) -> ErrorKind {
        let name = identifier.to_string();

        if treat_empty_as_unset {
            ErrorKind::UnsetOrEmpty { name }
        } else {
            ErrorKind::Unset { name }
        }
    }
}

fn map_first_char(value: &str, convert: impl Fn(char) -> String) -> String {
    let mut chars = value.chars();

    chars
        .next()
        .map_or_else(String::new, |c| convert(c) + chars.as_str())
}

fn reverse_case(c: char) -> String {
    if c.is_uppercase() {
        c.to_lowercase().to_string()
    } else {
        c.to_uppercase().to_string()
    }
}
