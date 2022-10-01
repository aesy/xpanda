use crate::ast::{Ast, Identifier, Node, Param};
use crate::parser::{self, Parser};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl Error {
    const fn new(message: String, line: usize, col: usize) -> Self {
        Self { message, line, col }
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

    pub fn eval(&self, ast: &Ast) -> Result<String, Error> {
        let mut result = String::new();

        for node in &ast.nodes {
            let text = self.eval_node(node)?;
            result.push_str(&text);
        }

        Ok(result)
    }

    fn eval_node(&self, node: &Node) -> Result<String, Error> {
        match node {
            Node::Text(text) => Ok((*text).to_string()),
            Node::Param(param) => self.eval_param(param),
        }
    }

    fn eval_param(&self, param: &Param) -> Result<String, Error> {
        match param {
            Param::Simple { identifier } => self.eval_simple_param(identifier),
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
            } => self.eval_error_param(identifier, error.clone(), *treat_empty_as_unset),
            Param::Length { identifier } => self.eval_length_param(identifier),
            Param::Arity => self.eval_arity_param(),
            Param::Ref { identifier } => self.eval_ref_param(identifier),
        }
    }

    fn eval_simple_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    // TODO wrong line/col
                    Err(Error::new(Self::error_message(identifier, false), 0, 0))
                } else {
                    Ok(String::from(""))
                }
            },
            Ok,
        )
    }

    fn eval_default_param(
        &self,
        identifier: &Identifier,
        default: &Node,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| self.eval_node(default), Ok)
    }

    fn eval_alt_param(
        &self,
        identifier: &Identifier,
        alt: &Node,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .map_or_else(|| Ok(String::from("")), |_| self.eval_node(alt))
    }

    fn eval_error_param(
        &self,
        identifier: &Identifier,
        error: Option<Cow<str>>,
        treat_empty_as_unset: bool,
    ) -> Result<String, Error> {
        self.eval_identifier(identifier)
            .filter(|value| !(treat_empty_as_unset && value.is_empty()))
            .ok_or_else(|| {
                let msg = error.map_or_else(
                    || Self::error_message(identifier, treat_empty_as_unset),
                    Cow::into_owned,
                );

                // TODO wrong line/col
                Error::new(msg, 0, 0)
            })
    }

    fn eval_length_param(&self, identifier: &Identifier) -> Result<String, Error> {
        self.eval_identifier(identifier).map_or_else(
            || {
                if self.no_unset {
                    // TODO wrong line/col
                    Err(Error::new(Self::error_message(identifier, false), 0, 0))
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
            .and_then(|name| self.eval_simple_param(&Identifier::Named(Cow::from(name))))
    }

    fn eval_identifier(&self, identifier: &Identifier) -> Option<String> {
        match identifier {
            Identifier::Named(name) => self.named_vars.get(name.as_ref()).cloned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_index() {
        let positional_vars = vec![String::from("woop")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Indexed(1)
            })])),
            Ok(String::from("woop"))
        );
    }

    #[test]
    fn simple_index_missing() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text(Cow::from("pre ")),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1)
                }),
                Node::Text(Cow::from(" post"))
            ])),
            Ok(String::from("pre  post"))
        );
    }

    #[test]
    fn simple_index_text() {
        let positional_vars = vec![String::from("woop")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text(Cow::from("pre ")),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1),
                }),
                Node::Text(Cow::from(" post"))
            ])),
            Ok(String::from("pre woop post"))
        );
    }

    #[test]
    fn simple_named() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Named(Cow::from("VAR"))
            })])),
            Ok(String::from("woop"))
        );
    }

    #[test]
    fn simple_named_missing() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text(Cow::from("pre ")),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("VAR"))
                }),
                Node::Text(Cow::from(" post"))
            ])),
            Ok(String::from("pre  post"))
        );
    }

    #[test]
    fn simple_named_text() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text(Cow::from("pre ")),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("VAR")),
                }),
                Node::Text(Cow::from(" post"))
            ])),
            Ok(String::from("pre woop post"))
        );
    }

    #[test]
    fn default_index() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Indexed(1),
                default: Box::new(Node::Text(Cow::from("default"))),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("default"))
        );
    }

    #[test]
    fn default_named() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named(Cow::from("VAR")),
                default: Box::new(Node::Text(Cow::from("default"))),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("default"))
        );
    }

    #[test]
    fn default_pattern() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("DEF"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named(Cow::from("VAR")),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("DEF")),
                })),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("woop"))
        );
    }

    #[test]
    fn default_index_no_empty() {
        let positional_vars = vec![(String::from(""))];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Indexed(1),
                default: Box::new(Node::Text(Cow::from("default"))),
                treat_empty_as_unset: true,
            })])),
            Ok(String::from("default"))
        );
    }

    #[test]
    fn default_named_no_empty() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named(Cow::from("VAR")),
                default: Box::new(Node::Text(Cow::from("default"))),
                treat_empty_as_unset: true,
            })])),
            Ok(String::from("default"))
        );
    }

    #[test]
    fn default_pattern_no_empty() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        named_vars.insert(String::from("DEF"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named(Cow::from("VAR")),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("DEF")),
                })),
                treat_empty_as_unset: true,
            })])),
            Ok(String::from("woop"))
        );
    }

    #[test]
    fn alt_index() {
        let positional_vars = vec![String::from("woop")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Indexed(1),
                alt: Box::new(Node::Text(Cow::from("alt"))),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("alt"))
        );
    }

    #[test]
    fn alt_named() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named(Cow::from("VAR")),
                alt: Box::new(Node::Text(Cow::from("alt"))),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("alt"))
        );
    }

    #[test]
    fn alt_pattern() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("woop"));
        named_vars.insert(String::from("ALT"), String::from("alt"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named(Cow::from("VAR")),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("ALT")),
                })),
                treat_empty_as_unset: false,
            })])),
            Ok(String::from("alt"))
        );
    }

    #[test]
    fn alt_index_no_empty() {
        let positional_vars = vec![String::from("")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Indexed(1),
                alt: Box::new(Node::Text(Cow::from("alt"))),
                treat_empty_as_unset: true,
            })])),
            Ok(String::from(""))
        );
    }

    #[test]
    fn alt_named_no_empty() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named(Cow::from("VAR")),
                alt: Box::new(Node::Text(Cow::from("alt"))),
                treat_empty_as_unset: true,
            })])),
            Ok(String::from(""))
        );
    }

    #[test]
    fn alt_pattern_no_empty() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        named_vars.insert(String::from("ALT"), String::from("alt"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named(Cow::from("VAR")),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named(Cow::from("ALT"))
                })),
                treat_empty_as_unset: true
            })])),
            Ok(String::from(""))
        );
    }

    #[test]
    fn error_index() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Indexed(1),
                error: Some(Cow::from("msg")),
                treat_empty_as_unset: false
            })])),
            Err(Error::new(String::from("msg"), 0, 0))
        );
    }

    #[test]
    fn error_named() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named(Cow::from("VAR")),
                error: Some(Cow::from("msg")),
                treat_empty_as_unset: false
            })])),
            Err(Error::new(String::from("msg"), 0, 0))
        );
    }

    #[test]
    fn error_index_no_empty() {
        let positional_vars = vec![String::from("")];
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Indexed(1),
                error: Some(Cow::from("msg")),
                treat_empty_as_unset: true
            })])),
            Err(Error::new(String::from("msg"), 0, 0))
        );
    }

    #[test]
    fn error_named_no_empty() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named(Cow::from("VAR")),
                error: Some(Cow::from("msg")),
                treat_empty_as_unset: true
            })])),
            Err(Error::new(String::from("msg"), 0, 0))
        );
    }

    #[test]
    fn error_no_message() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named(Cow::from("VAR")),
                error: None,
                treat_empty_as_unset: false
            })])),
            Err(Error::new(String::from("'VAR' is unset"), 0, 0))
        );
    }

    #[test]
    fn error_no_message_no_empty() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from(""));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named(Cow::from("VAR")),
                error: None,
                treat_empty_as_unset: true
            })])),
            Err(Error::new(String::from("'VAR' is unset or empty"), 0, 0))
        );
    }

    #[test]
    fn len_index() {
        let positional_vars = vec![String::from("four")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Length {
                identifier: Identifier::Indexed(1)
            })])),
            Ok(String::from("4"))
        );
    }

    #[test]
    fn len_named() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("four"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Length {
                identifier: Identifier::Named(Cow::from("VAR"))
            })])),
            Ok(String::from("4"))
        );
    }

    #[test]
    fn len_missing() {
        let positional_vars = Vec::new();
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Length {
                identifier: Identifier::Named(Cow::from("VAR"))
            })])),
            Ok(String::from("0"))
        );
    }

    #[test]
    fn arity() {
        let positional_vars = vec![String::from("one"), String::from("two")];
        let named_vars = HashMap::new();
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Arity)])),
            Ok(String::from("2"))
        );
    }

    #[test]
    fn ref_index() {
        let positional_vars = vec![String::from("VAR")];
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Ref {
                identifier: Identifier::Indexed(1)
            })])),
            Ok(String::from("woop"))
        );
    }

    #[test]
    fn ref_named() {
        let positional_vars = Vec::new();
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR1"), String::from("VAR2"));
        named_vars.insert(String::from("VAR2"), String::from("woop"));
        let mut evaluator = Evaluator::new(false, positional_vars, named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Ref {
                identifier: Identifier::Named(Cow::from("VAR1"))
            })])),
            Ok(String::from("woop"))
        );
    }
}
