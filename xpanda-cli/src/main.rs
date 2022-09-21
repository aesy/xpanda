#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

mod args;
mod read;

use crate::args::Args;
use crate::read::read_var_file;
use clap::Parser;
use std::io::{self, Write};
use std::process::ExitCode;
use xpanda::Xpanda;

fn main() -> ExitCode {
    let stdin = io::stdin().lines();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    let Args {
        no_unset,
        var_file,
        env_vars,
        named_vars,
        positional_vars,
    } = Args::parse();
    let has_user_provided_vars =
        var_file.is_some() || !named_vars.is_empty() || !positional_vars.is_empty();
    let mut builder = Xpanda::builder().no_unset(no_unset);

    match env_vars {
        Some(true) | None if !has_user_provided_vars => {
            builder = builder.with_env_vars();
        },
        _ => {},
    }

    if let Some(file) = var_file {
        let file_vars = match read_var_file(&file) {
            Ok(vars) => vars,
            Err(error) => {
                stderr
                    .write_all(format!("Failed to read var file: {:?}", error).as_bytes())
                    .unwrap();
                return ExitCode::from(1);
            },
        };

        builder = builder.with_named_vars(file_vars);
    }

    let xpanda = builder
        .with_positional_vars(positional_vars)
        .with_named_vars(named_vars.into_iter().collect())
        .build();

    // Rule disabled because it's a false positive
    // See https://github.com/rust-lang/rust-clippy/issues/9135
    #[allow(clippy::significant_drop_in_scrutinee)]
    for line in stdin {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                stderr
                    .write_all(format!("Failed to read stdin: {:?}", error).as_bytes())
                    .unwrap();
                return ExitCode::from(1);
            },
        };

        let text = match xpanda.expand(&line) {
            Ok(text) => text,
            Err(error) => {
                stderr
                    .write_all(format!("Failed to expand text: {:?}", error).as_bytes())
                    .unwrap();
                return ExitCode::from(1);
            },
        };

        if let Err(error) = stdout.write_all(text.as_bytes()) {
            stderr
                .write_all(format!("Failed to write to stdout: {:?}", error).as_bytes())
                .unwrap();
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}
