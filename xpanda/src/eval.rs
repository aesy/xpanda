use crate::ast::{Ast, Identifier, Node, Param};
use crate::parser::{Error, Parser};
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_index() {
        let positional_vars = vec![String::from("one 2")];
        let mut evaluator = Evaluator::new(false, positional_vars, HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Indexed(1)
            })])),
            String::from("one 2")
        );
    }

    #[test]
    fn simple_index_missing() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("abc "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1)
                }),
                Node::Text(" def")
            ])),
            String::from("abc  def")
        );
    }

    #[test]
    fn simple_index_text() {
        let mut named_vars = HashMap::new();
        named_vars.insert(String::from("VAR"), String::from("one 2"));
        let mut evaluator = Evaluator::new(false, Vec::new(), named_vars);

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1),
                }),
                Node::Text(" post")
            ])),
            String::from("")
        );
    }

    #[test]
    fn simple_named_missing() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("abc "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named("VAR")
                }),
                Node::Text(" def")
            ])),
            String::from("abc  def")
        );
    }

    #[test]
    fn simple_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Named("VAR")
            })])),
            String::from("")
        );
    }

    #[test]
    fn simple_named_text() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named("VAR"),
                }),
                Node::Text(" post")
            ])),
            String::from("")
        );
    }

    #[test]
    fn braced_index() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Indexed(1),
            })])),
            String::from("")
        );
    }

    #[test]
    fn braced_index_text() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Indexed(1),
                }),
                Node::Text(" post")
            ])),
            String::from("")
        );
    }

    #[test]
    fn braced_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Simple {
                identifier: Identifier::Named("VAR"),
            })])),
            String::from("")
        );
    }

    #[test]
    fn braced_named_text() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![
                Node::Text("pre "),
                Node::Param(Param::Simple {
                    identifier: Identifier::Named("VAR"),
                }),
                Node::Text(" post"),
            ])),
            String::from("")
        );
    }

    #[test]
    fn default_index() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Indexed(1),
                default: Box::new(Node::Text("default")),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn default_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Text("default")),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn default_pattern() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("DEF"),
                })),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn default_index_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Indexed(1),
                default: Box::new(Node::Text("default")),
                treat_empty_as_unset: true,
            })])),
            String::from("")
        );
    }

    #[test]
    fn default_named_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Text("default")),
                treat_empty_as_unset: true,
            })])),
            String::from("")
        );
    }

    #[test]
    fn default_pattern_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithDefault {
                identifier: Identifier::Named("VAR"),
                default: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("DEF"),
                })),
                treat_empty_as_unset: true,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_index() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Indexed(1),
                alt: Box::new(Node::Text("alt")),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Text("alt")),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_pattern() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("ALT"),
                })),
                treat_empty_as_unset: false,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_index_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Indexed(1),
                alt: Box::new(Node::Text("alt")),
                treat_empty_as_unset: true,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_named_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Text("alt")),
                treat_empty_as_unset: true,
            })])),
            String::from("")
        );
    }

    #[test]
    fn alt_pattern_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithAlt {
                identifier: Identifier::Named("VAR"),
                alt: Box::new(Node::Param(Param::Simple {
                    identifier: Identifier::Named("ALT")
                })),
                treat_empty_as_unset: true
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_index() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Indexed(1),
                error: Some("msg"),
                treat_empty_as_unset: false
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named("VAR"),
                error: Some("msg"),
                treat_empty_as_unset: false
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_index_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Indexed(1),
                error: Some("msg"),
                treat_empty_as_unset: true
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_named_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named("VAR"),
                error: Some("msg"),
                treat_empty_as_unset: true
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_no_message() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named("VAR"),
                error: None,
                treat_empty_as_unset: false
            })])),
            String::from("")
        );
    }

    #[test]
    fn error_no_message_no_empty() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::WithError {
                identifier: Identifier::Named("VAR"),
                error: None,
                treat_empty_as_unset: true
            })])),
            String::from("")
        );
    }

    #[test]
    fn len_index() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Length {
                identifier: Identifier::Indexed(1)
            })])),
            String::from("")
        );
    }

    #[test]
    fn len_named() {
        let mut evaluator = Evaluator::new(false, Vec::new(), HashMap::new());

        assert_eq!(
            evaluator.eval(&Ast::new(vec![Node::Param(Param::Length {
                identifier: Identifier::Named("VAR")
            })])),
            String::from("")
        );
    }
}
