/*!
This crate provides the ability to expand/substitute variables in strings similar to [`envsubst`]
and [`Bash parameter expansion`].

There is a single public struct (not counting errors and builders), [`Xpanda`], which in turn
contains a single method: `expand`. The expand method takes a string by reference and returns
a copy of it with all variables expanded/substituted according to some patterns.

[`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html
[`Bash parameter expansion`]: https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html
[`Xpanda`]: struct.Xpanda.html
*/

#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(unused)]

mod ast;
mod eval;
mod forward_peekable;
mod lexer;
mod parser;
mod position;
mod str_read;
mod token;

use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position::Position;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl Error {
    #[must_use]
    pub const fn new(message: String, position: &Position) -> Self {
        Self {
            message,
            line: position.line,
            col: position.col,
        }
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Self::new(error.message, &error.position)
    }
}

impl From<eval::Error> for Error {
    fn from(error: eval::Error) -> Self {
        Self::new(error.message, &Position::default())
    }
}

#[derive(Default)]
pub struct Builder {
    no_unset: bool,
    positional_vars: Vec<String>,
    named_vars: HashMap<String, String>,
}

impl Builder {
    /// With this flag set, missing variables without any default value will cause an error
    /// instead of omitting en empty string. Off by default.
    #[must_use]
    pub const fn no_unset(mut self, no_unset: bool) -> Self {
        self.no_unset = no_unset;
        self
    }

    /// Adds all environment variables as named variables.
    #[must_use]
    pub fn with_env_vars(mut self) -> Self {
        self.named_vars.extend(env::vars());
        self
    }

    /// Adds the given map values as named variables.
    #[must_use]
    pub fn with_named_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.named_vars.extend(vars);
        self
    }

    /// Adds the given strings as positional variables.
    #[must_use]
    pub fn with_positional_vars(mut self, vars: Vec<String>) -> Self {
        self.positional_vars.extend(vars);
        self
    }

    /// Builds a new [`Xpanda`] instance.
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
    /// Variables can appear in any of the following forms:
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th>Pattern</th>
    ///       <th>Description</th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td>$VAR</td>
    ///       <td>substituted with the corresponding value for 'VAR' if set, otherwise "".</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR}</td>
    ///       <td>substituted with the corresponding value for 'VAR' if set, otherwise "".</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR-default}</td>
    ///       <td>
    ///         substituted with the corresponding value for 'VAR' if set, otherwise "default".
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR:-default}</td>
    ///       <td>
    ///         substituted with the corresponding value for 'VAR' if set and non-empty, otherwise
    ///         "default".
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR+alternative}</td>
    ///       <td>
    ///         substituted with "alternative" if the corresponding value for 'VAR' is set,
    ///         otherwise "".
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR:+alternative}</td>
    ///       <td>
    ///         substituted with "alternative" if the corresponding value for 'VAR' is set and
    ///         non-empty, otherwise "".
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR?}</td>
    ///       <td>
    ///         substituted with the corresponding value for 'VAR' if set, otherwise yields an
    ///         error.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR?error}</td>
    ///       <td>
    ///         substituted with the corresponding value for 'VAR' if set, otherwise yields an
    ///         error with the given message (in this case "error").
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR?error}</td>
    ///       <td>
    ///         substituted with the corresponding value for 'VAR' if set and non-empty, otherwise
    ///         yields an error with the given message (in this case "error").
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${#VAR}</td>
    ///       <td>
    ///         substituted with the length of the corresponding value for 'VAR' if set, otherwise
    ///         "0".
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with the
    ///         first character uppercased.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^^}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with all
    ///         characters uppercased.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with the
    ///         first character lowercased.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,,}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with all
    ///         characters lowercased.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with the
    ///         casing of the first character reversed.
    ///       </td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~~}</td>
    ///       <td>
    ///         substituted with the value of the variable named by the value of `VAR`, with the
    ///         casing of all characters reversed.
    ///       </td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ///
    /// `VAR` above is a named variable. Named variables can be provided using the builder:
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use xpanda::Xpanda;
    ///
    /// let named_vars = HashMap::new();
    /// let xpanda = Xpanda::builder()
    ///     .with_named_vars(named_vars)
    ///     .build();
    /// ```
    ///
    /// Positional variables are also supported and can be provided in the same way:
    ///
    /// ```rust
    /// use xpanda::Xpanda;
    ///
    /// let xpanda = Xpanda::builder()
    ///     .with_positional_vars(Vec::new())
    ///     .build();
    /// ```
    ///
    /// Positional variables can be referenced using their index (starting at 1), for example, `$1`
    /// references the first positional variable, `$2` the second and so on. `$0` is a space concatenated
    /// string of all positional variables.
    ///
    /// Here are some examples and their output:
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th>Pattern</th>
    ///       <th>VAR unset</th>
    ///       <th>VAR=""</th>
    ///       <th>VAR="example"</th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td>$VAR</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR-default}</td>
    ///       <td>"default"</td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR:-default}</td>
    ///       <td>"default"</td>
    ///       <td>"default"</td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR+alternative}</td>
    ///       <td></td>
    ///       <td>"alternative"</td>
    ///       <td>"alternative"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR:+alternative}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"alternative"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR?message}</td>
    ///       <td>error: "message"</td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR:?message}</td>
    ///       <td>error: "message"</td>
    ///       <td>error: "message"</td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${#VAR}</td>
    ///       <td>"0"</td>
    ///       <td>"0"</td>
    ///       <td>"7"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${!VAR}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>$example</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"Example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^^}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"EXAMPLE"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,,}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"Example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~~}</td>
    ///       <td></td>
    ///       <td></td>
    ///       <td>"EXAMPLE"</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ///
    /// Special rules take precedence when [`Builder::no_unset`] is `true`:
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th>Pattern</th>
    ///       <th>VAR unset</th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td>$VAR</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${#VAR}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${!VAR}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR^^}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR,,}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~}</td>
    ///       <td>error</td>
    ///     </tr>
    ///     <tr>
    ///       <td>${VAR~~}</td>
    ///       <td>error</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ///
    /// Default/Alternative values can also be variables:
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th>Pattern</th>
    ///       <th>VAR unset</th>
    ///       <th>VAR=""</th>
    ///       <th>VAR="example"</th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td>`${VAR:-$DEF}`</td>
    ///       <td>`$DEF`</td>
    ///       <td></td>
    ///       <td>"example"</td>
    ///     </tr>
    ///     <tr>
    ///       <td>`${VAR+${ALT:-alternative}}`</td>
    ///       <td></td>
    ///       <td>`${ALT:-alternative}`</td>
    ///       <td>${ALT:-alternative}</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ///
    /// The `$` character is assumed to be the start of a variable. If the variable does not match
    /// any of the forms listed above, an error is returned. Variables can be escaped by prefixing them
    /// by an additional '$', for example: `$$VAR` which yields `$VAR` and `${VAR-$$text}` which yields
    /// `$text` if `VAR` is unset.
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
    /// let xpanda = Xpanda::default();
    /// assert_eq!(xpanda.expand("${1:-default}"), Ok(String::from("default")));
    /// ```
    pub fn expand(&self, input: &str) -> Result<String, Error> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse()?;
        let result = self.evaluator.eval(ast)?;

        Ok(result)
    }
}
