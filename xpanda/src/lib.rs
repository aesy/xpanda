#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(unused)]

mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Self {
            message: error.message,
            line: error.line,
            col: error.col,
        }
    }
}

impl From<eval::Error> for Error {
    fn from(error: eval::Error) -> Self {
        Self {
            message: error.message,
            line: error.line,
            col: error.col,
        }
    }
}

#[derive(Default)]
pub struct Builder {
    no_unset: bool,
    positional_vars: Vec<String>,
    named_vars: HashMap<String, String>,
}

impl Builder {
    #[must_use]
    pub const fn no_unset(mut self, no_unset: bool) -> Self {
        self.no_unset = no_unset;
        self
    }

    #[must_use]
    pub fn with_env_vars(mut self) -> Self {
        self.named_vars.extend(env::vars());
        self
    }

    #[must_use]
    pub fn with_named_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.named_vars.extend(vars);
        self
    }

    #[must_use]
    pub fn with_positional_vars(mut self, vars: Vec<String>) -> Self {
        self.positional_vars.extend(vars);
        self
    }

    #[must_use]
    pub fn build(self) -> Xpanda {
        Xpanda::new(self)
    }
}

/// [`Xpanda`] substitutes the values of variables in strings similar to [`envsubst`] and
/// [`Bash parameter expansion`].
///
/// [`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html
/// [`Bash parameter expansion`]: https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html
#[derive(Default)]
pub struct Xpanda {
    evaluator: Evaluator,
}

impl Xpanda {
    fn new(builder: Builder) -> Self {
        Self {
            evaluator: Evaluator::new(
                builder.no_unset,
                builder.positional_vars,
                builder.named_vars,
            ),
        }
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Expands the given text by substituting the values of the variables inside it.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the given string is badly formatted and cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpanda::Xpanda;
    ///
    /// let mut xpanda = Xpanda::default();
    /// assert_eq!(xpanda.expand("${1:-default}"), Ok(String::from("default")));
    /// ```
    pub fn expand(&self, input: &str) -> Result<String, Error> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse()?;
        let result = self.evaluator.eval(&ast)?;

        Ok(result)
    }
}
