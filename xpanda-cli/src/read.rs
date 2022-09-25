#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Tries to read a string in key=value format, returning the key and value as a tuple
/// (in that order).
pub fn read_named_arg(arg: &str) -> Result<(String, String), String> {
    match arg.rsplit_once('=') {
        Some((key, value)) => Ok((key.to_string(), value.to_string())),
        None => Err(String::from("'=' character missing in key value pair")),
    }
}

/// Reads a file of key=value pairs, ignoring empty lines.
pub fn read_var_file(path: &Path) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "Failed to open var file '{}': {}",
                path.display(),
                error
            ))
        },
    };

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                return Err(format!(
                    "Failed to read var file '{}': {}",
                    path.display(),
                    error
                ))
            },
        };

        if line.trim().is_empty() {
            continue;
        }

        match read_named_arg(&line) {
            Ok((key, value)) => {
                map.insert(key, value);
            },
            Err(error) => return Err(format!("Failed to parse named arg: {}", error)),
        }
    }

    Ok(map)
}

/// Reads the next line from stdin just like [`std::io::Lines::next`] except that it includes
/// the line ending in the returned string.
pub fn read_line(buf: &mut impl BufRead) -> Option<Result<String, io::Error>> {
    let mut string = String::new();

    #[allow(clippy::significant_drop_in_scrutinee)]
    match buf.read_line(&mut string) {
        Ok(0) => None,
        Ok(_) => Some(Ok(string)),
        Err(e) => Some(Err(e)),
    }
}
