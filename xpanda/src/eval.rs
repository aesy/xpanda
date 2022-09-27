use crate::ast::{Ast, Identifier, Node, Param};
use crate::parser::{self, Parser};
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
        fn error_message(identifier: &Identifier, treat_empty_as_unset: bool) -> String {
            if treat_empty_as_unset {
                format!("'{}' is unset or empty", identifier)
            } else {
                format!("'{}' is unset", identifier)
            }
        }

        match param {
            Param::Simple { identifier } => {
                self.eval_identifier(identifier, false).map_or_else(
                    || {
                        if self.no_unset {
                            // TODO wrong line/col
                            Err(Error::new(error_message(identifier, false), 0, 0))
                        } else {
                            Ok(String::from(""))
                        }
                    },
                    Ok,
                )
            },
            Param::Length { identifier } => {
                self.eval_identifier(identifier, false).map_or_else(
                    || {
                        if self.no_unset {
                            // TODO wrong line/col
                            Err(Error::new(error_message(identifier, false), 0, 0))
                        } else {
                            Ok(String::from("0"))
                        }
                    },
                    |value| Ok(value.len().to_string()),
                )
            },
            Param::WithDefault {
                identifier,
                default,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, *treat_empty_as_unset)
                .map_or_else(|| self.eval_node(default), Ok),
            Param::WithAlt {
                identifier,
                alt,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, !*treat_empty_as_unset)
                .map_or_else(|| Ok(String::from("")), |_| self.eval_node(alt)),
            Param::WithError {
                identifier,
                error,
                treat_empty_as_unset,
            } => self
                .eval_identifier(identifier, *treat_empty_as_unset)
                .ok_or_else(|| {
                    let msg = error
                        .map(str::to_string)
                        .unwrap_or_else(|| error_message(identifier, *treat_empty_as_unset));

                    // TODO wrong line/col
                    Error::new(msg, 0, 0)
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
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1)
                }),
                Node::Text(" post")
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
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1),
                }),
                Node::Text(" post")
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
                identifier: Identifier::Named("VAR")
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
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named("VAR")
                }),
                Node::Text(" post")
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
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named("VAR"),
                }),
                Node::Text(" post")
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
                default: Box::new(Node::Text("default")),
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
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Text("default")),
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
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("DEF"),
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
                default: Box::new(Node::Text("default")),
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
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Text("default")),
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
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("DEF"),
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
                alt: Box::new(Node::Text("alt")),
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
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Text("alt")),
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
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("ALT"),
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
                alt: Box::new(Node::Text("alt")),
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
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Text("alt")),
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
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("ALT")
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
                error: Some("msg"),
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
                identifier: Identifier::Named("VAR"),
                error: Some("msg"),
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
                error: Some("msg"),
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
                identifier: Identifier::Named("VAR"),
                error: Some("msg"),
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
                identifier: Identifier::Named("VAR"),
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
                identifier: Identifier::Named("VAR"),
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
                identifier: Identifier::Named("VAR")
            })])),
            Ok(String::from("4"))
        );
    }
}
