use crate::ast::{Ast, Identifier, Node, Param};
use crate::parser::{ParseError, Parser};
use std::collections::HashMap;

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

    pub fn eval(&self, ast: &Ast) -> String {
        let mut result = String::new();

        for node in &ast.nodes {
            let text = self.eval_node(node);
            result.push_str(&text);
        }

        result
    }

    fn eval_node(&self, node: &Node) -> String {
        match node {
            Node::Text(text) => (*text).to_string(),
            Node::Param(param) => self.eval_param(param),
        }
    }

    fn eval_param(&self, param: &Param) -> String {
        match param {
            Param::Simple { identifier } => self
                .eval_identifier(identifier, self.no_unset)
                .unwrap_or_else(|| String::from("")),
            Param::Length { identifier } => {
                self.eval_identifier(identifier, self.no_unset).map_or_else(
                    || String::from("0"),
                    |identifier| identifier.len().to_string(),
                )
            },
            Param::WithDefault {
                identifier,
                default,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, *treat_empty_as_unset)
                .unwrap_or_else(|| self.eval_node(default)),
            Param::WithAlt {
                identifier,
                alt,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, *treat_empty_as_unset)
                .map_or_else(|| String::from(""), |_| self.eval_node(alt)),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, *treat_empty_as_unset)
                .unwrap_or_else(|| {
                    error.map(str::to_string).unwrap_or_else(|| {
                        if *treat_empty_as_unset {
                            format!("{:?} is unset or empty", identifier)
                        } else {
                            format!("{:?} is unset", identifier)
                        }
                    })
                }),
        }
    }

    fn eval_identifier(
        &self,
        identifier: &Identifier,
        treat_empty_as_unset: bool,
    ) -> Option<String> {
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
        .filter(|value| !(treat_empty_as_unset && value.is_empty()))
    }
}
