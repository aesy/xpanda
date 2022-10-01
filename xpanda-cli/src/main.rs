#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

mod args;
mod read;

use crate::args::Args;
use crate::read::{read_input_file, read_line, read_output_file, read_var_file};
use clap::Parser;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use xpanda::Xpanda;

fn main() -> ExitCode {
    let mut stderr = io::stderr().lock();
    let Args {
        no_unset,
        var_files,
        env_vars,
        named_vars,
        positional_vars,
        input_file,
        output_file,
    } = Args::parse();
    let has_user_provided_vars =
        !var_files.is_empty() || !named_vars.is_empty() || !positional_vars.is_empty();
    let mut builder = Xpanda::builder().no_unset(no_unset);

    if env_vars == Some(true) || (env_vars.is_none() && !has_user_provided_vars) {
        builder = builder.with_env_vars();
    }

    for var_file in var_files {
        let file_vars = match read_var_file(&var_file) {
            Ok(vars) => vars,
            Err(error) => {
                let _result = stderr.write_all(error.as_bytes());
                return ExitCode::from(1);
            },
        };

        builder = builder.with_named_vars(file_vars);
    }

    let xpanda = builder
        .with_positional_vars(positional_vars)
        .with_named_vars(named_vars.into_iter().collect())
        .build();

    let mut input: Box<dyn BufRead> = if let Some(path) = input_file {
        match read_input_file(&path) {
            Ok(file) => Box::new(file),
            Err(error) => {
                let _result = stderr.write_all(error.as_bytes());
                return ExitCode::from(1);
            },
        }
    } else {
        Box::new(io::stdin().lock())
    };

    let mut output: Box<dyn Write> = if let Some(path) = output_file {
        match read_output_file(&path) {
            Ok(file) => Box::new(file),
            Err(error) => {
                let _result = stderr.write_all(error.as_bytes());
                return ExitCode::from(1);
            },
        }
    } else {
        Box::new(io::stdout().lock())
    };

    let mut line_number = 0;
    while let Some(line) = read_line(&mut input) {
        line_number += 1;

        let line = match line {
            Ok(line) => line,
            Err(error) => {
                let _result = stderr.write_all(error.as_bytes());
                return ExitCode::from(1);
            },
        };

        let text = match xpanda.expand(&line) {
            Ok(text) => text,
            Err(error) => {
                let _result = stderr.write_all(
                    format!("{}:{} {}", line_number, error.col, error.message).as_bytes(),
                );
                return ExitCode::from(1);
            },
        };

        if let Err(error) = output.write_all(text.as_bytes()) {
            let _result = stderr.write_all(format!("Failed to write output: {}", error).as_bytes());
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}
