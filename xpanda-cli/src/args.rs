use crate::read::read_named_arg;
use clap::Parser;
use std::path::PathBuf;

/// Unix shell-like parameter expansion/variable substitution.
///
/// This program will process some input and copy it to some output with all variables
/// expanded/substituted using the provided variables. If no variables are provided, then
/// values are sourced from environment variables.
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
/// ${#}                substituted with number of positional variables.
/// ${!VAR}             substituted with the value of the variable named by the value of `VAR`.
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
/// `echo '$VAR' | xpanda -v VAR=value`   substitute `$VAR` with `value` using a named variable
///                                       argument.
/// `VAR=value echo '$VAR' | xpanda`      substitute `$VAR` with `value` using an environment
///                                       variable.
/// `echo '$1' | xpanda value`            substitute `$1` with `value` using a positional variable
///                                       argument.
/// `xpanda < some_file`                  output a copy of `some_file` with variables substituted
///                                       with environment variables.
/// `xpanda -f var_file < some_file`      output a copy of `some_file` with variables substituted
///                                       with variables from `var_file`.
/// `xpanda -v VAR=value < some_file`     output a copy of `some_file` with `$VAR` substituted with
///                                       `value` using `-v`.
///
/// The given input must be ASCII or UTF-8 encoded. Output is UTF-8 encoded and may be written
/// in chunks.
#[derive(Parser, Debug)]
#[command(name = "Xpanda", version, verbatim_doc_comment)]
pub struct Args {
    /// With this flag set, missing variables without any default value will cause the program
    /// to exit with a status code of 1. Off by default.
    #[arg(long = "no-unset", short = 'u', verbatim_doc_comment)]
    pub no_unset: bool,

    /// Provide a file to source variable values from.
    ///
    /// This option can be used multiple times in order to add multiple files.
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
    #[arg(
        long = "var-file",
        short = 'f',
        num_args = 1,
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        verbatim_doc_comment
    )]
    pub var_files: Vec<PathBuf>,

    /// With this flag set, named variables will be sourced from environment variables in
    /// addition to any other provided variables. Named variables will always take precedence
    /// over environment variables though. This flag is implicitly true if no other variables
    /// are provided.
    #[arg(
        long = "env-vars",
        short = 'e',
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "true",
        verbatim_doc_comment
    )]
    pub env_vars: Option<bool>,

    /// Adds a named variable to source from. The value should be a key value pair separated
    /// by a `=`, e.g. `-v NAME=value`.
    ///
    /// This option can be used multiple times in order to add multiple variables.
    ///
    /// Using this option will override the default setting to source values from environment
    /// variables. To continue sourcing from environment values as well, add the `--env-vars`
    /// flag.
    #[arg(
        long = "var",
        short = 'v',
        value_name = "VAR",
        num_args = 1,
        value_parser = read_named_arg,
        verbatim_doc_comment
    )]
    pub named_vars: Vec<(String, String)>,

    /// Zero or more positional variable values. The first value can be referenced using `$1`,
    /// the second `$2` and so on.
    ///
    /// If any positional variables are provided then the default setting to source values
    /// from environment variables will be overridden. To continue sourcing from environment
    /// values as well, add the `--env-vars` flag.
    #[arg(last = true, num_args = 0.., verbatim_doc_comment)]
    pub positional_vars: Vec<String>,

    /// Provide a path to read from. This overrides the default behaviour of reading from
    /// standard input.
    #[arg(
        long = "input",
        short = 'i',
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        verbatim_doc_comment
    )]
    pub input_file: Option<PathBuf>,

    /// Provide a path to write to. This overrides the default behaviour of writing to
    /// standard output. A new file is created if it doesn't already exists. Output is
    /// appended to it if it already exists.
    #[arg(
        long = "output",
        short = 'o',
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        verbatim_doc_comment
    )]
    pub output_file: Option<PathBuf>,
}
