#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// Tries to read a string in key=value format, returning the key and value as a tuple
/// (in that order).
pub fn read_named_arg(arg: &str) -> Result<(String, String), String> {
    arg.rsplit_once('=')
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .ok_or_else(|| String::from("'=' character missing in key value pair"))
}

/// Reads a file of key=value pairs, ignoring empty lines.
pub fn read_var_file(path: &Path) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    let file = File::open(path)
        .map(BufReader::new)
        .map_err(|error| format!("Failed to open var file '{}': {}", path.display(), error))?;

    for line in file.lines() {
        let line = line
            .map_err(|error| format!("Failed to read var file '{}': {}", path.display(), error))?;

        if line.trim().is_empty() {
            continue;
        }

        let (key, value) = read_named_arg(&line)
            .map_err(|error| format!("Failed to parse named arg: {}", error))?;

        map.insert(key, value);
    }

    Ok(map)
}

pub fn read_input_file(path: &Path) -> Result<impl BufRead, String> {
    File::open(path)
        .map(BufReader::new)
        .map_err(|error| format!("Failed to open input file '{}': {}", path.display(), error))
}

pub fn read_output_file(path: &Path) -> Result<impl Write, String> {
    OpenOptions::new()
        .write(true)
        .create_new(!path.exists())
        .append(true)
        .open(path)
        .map(BufWriter::new)
        .map_err(|error| format!("Failed to open output file '{}': {}", path.display(), error))
}

/// Reads the next line from stdin just like [`std::io::Lines::next`] except that it includes
/// the line ending in the returned string.
pub fn read_line(buf: &mut impl BufRead) -> Option<Result<String, String>> {
    let mut string = String::new();

    #[allow(clippy::significant_drop_in_scrutinee)]
    match buf.read_line(&mut string) {
        Ok(0) => None,
        Ok(_) => Some(Ok(string)),
        Err(error) => Some(Err(format!("Failed to read input: {}", error))),
    }
}
