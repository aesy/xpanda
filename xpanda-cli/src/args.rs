use crate::read::read_named_arg;
use clap::Parser;
use std::path::PathBuf;

/// Unix shell-like parameter expansion/variable substitution.
///
/// This program will process standard input and copy it to standard output with all variables
/// expanded/substituted using the provided variables. If no variables are provided, then values
/// are sourced from environment variables.
///
/// Variables can appear in the input in any of the following forms:
/// $VAR                substituted with the corresponding value for `VAR` if set, otherwise ``.
/// ${VAR}              substituted with the corresponding value for `VAR` if set, otherwise ``.
/// ${VAR-default}      substituted with the corresponding value for `VAR` if set, otherwise
///                     `default`.
/// ${VAR:-default}     substituted with the corresponding value for `VAR` if set and non-empty,
///                     otherwise `default`.
/// ${VAR+alternative}  substituted with `alternative` if the corresponding value for `VAR` is
///                     set, otherwise ``.
/// ${VAR:+alternative} substituted with `alternative` if the corresponding value for `VAR` is
///                     set and non-empty, otherwise ``.
/// ${VAR?}             substituted with the corresponding value for `VAR` if set, otherwise
///                     causes the program to exit with a status code of 1 and an error
///                     printed to standard error.
/// ${VAR?error}        substituted with the corresponding value for `VAR` if set, otherwise
///                     causes the program to exit with a status code of 1 and `error`
///                     printed to standard error.
/// ${VAR?error}        substituted with the corresponding value for `VAR` if set and non-empty,
///                     otherwise causes the program to exit with a status code of 1 and `error`
///                     printed to standard error.
/// ${#VAR}             substituted with the length of the corresponding value for `VAR` if set,
///                     otherwise `0`.
///
/// `VAR` above is a named variable. Positional variables are also supported and are passed as
/// trailing arguments to the program (see the examples). They can be referenced using their
/// index (starting at 1), for example, `$1` references the first positional variable, `$2` the
/// second and so on. `$0` is a space concatenated string of all positional variables.
///
/// The `$` character is assumed to be the start of a variable. If the variable does not match
/// any of the forms listed above, the program will fail to parse the variable and exit the
/// program with a status code of 1.
///
/// EXAMPLES:
/// `echo '$VAR' | xpanda -v VAR=value`   substitute `$VAR` with `value` using `-v`.
/// `VAR=value echo '$VAR' | xpanda`      substitute `$VAR` with `value` using an environment
///                                       variable.
/// `echo '$1' | xpanda value`            substitute `$1` with `value` using a positional variable.
/// `cat some_file | xpanda`              output a copy of `some_file` with variables substituted
///                                       with environment variables.
/// `cat some_file | xpanda -f var_file`  output a copy of `some_file` with variables substituted
///                                       with variables from `var_file`.
/// `cat some_file | xpanda -v VAR=value` output a copy of `some_file` with `$VAR` substituted with
///                                       `value` using `-v`.
#[derive(Parser, Debug)]
#[clap(name = "Xpanda", version, verbatim_doc_comment)]
pub struct Args {
    /// With this flag set, missing variables without any default value will cause the program
    /// to exit with a status code of 1.
    #[clap(long = "no-unset", short = 'u', verbatim_doc_comment)]
    pub no_unset: bool,

    /// Provide a file to source variable values from.
    ///
    /// Using this option will override the default setting to source values from environment
    /// variables. To continue sourcing from environment values as well, add the `--env-vars`
    /// flag.
    ///
    /// The file must be formatted as key=value pairs with one variable per line. Failure to
    /// parse this file will cause the program to exit with status code 1.
    ///
    /// Example:
    /// KEY1=value
    /// KEY2=value
    #[clap(long = "var-file", short = 'f', value_hint = clap::ValueHint::FilePath, verbatim_doc_comment)]
    pub var_file: Option<PathBuf>,

    /// With this flag set, named variables will be sourced from environment variables in addition
    /// to any other provided variables. Named variables will always take precedence over
    /// environment variables though. This flag is implicitly true if unset and no other variables
    /// are set.
    #[clap(long = "env-vars", short = 'e', verbatim_doc_comment)]
    pub env_vars: Option<bool>,

    /// Adds a named variable to source from. The value should be a key value pair separated
    /// by a `=`, e.g. `-v NAME=value`.
    ///
    /// This option can be used multiple times in order to add multiple variables.
    ///
    /// Using this option will override the default setting to source values from environment
    /// variables. To continue sourcing from environment values as well, add the `--env-vars`
    /// flag.
    #[clap(
        long = "var",
        short = 'v',
        multiple = true,
        number_of_values = 1,
        parse(try_from_str = read_named_arg),
        verbatim_doc_comment
    )]
    pub named_vars: Vec<(String, String)>,

    /// Zero or more positional variable values. The first value can be substituted using `$1`,
    /// the second `$2` and so on.
    ///
    /// If any positional variables are provided then the default setting to source values
    /// from environment variables will be overridden. To continue sourcing from environment
    /// values as well, add the `--env-vars` flag.
    #[clap(multiple = true, verbatim_doc_comment)]
    pub positional_vars: Vec<String>,
}
