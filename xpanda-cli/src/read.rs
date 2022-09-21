#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_named_arg(arg: &str) -> Result<(String, String), String> {
    match arg.rsplit_once('=') {
        Some((key, value)) => Ok((key.to_string(), value.to_string())),
        None => Err(String::from("'=' character missing in key value pair")),
    }
}

pub fn read_var_file(file: &Path) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    let file = match File::open(file) {
        Ok(file) => file,
        Err(error) => return Err(format!("Failed to open var file: {:?}", error)),
    };

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return Err(format!("Failed to read var file: {:?}", error)),
        };

        if line.trim().is_empty() {
            continue;
        }

        match read_named_arg(&line) {
            Ok((key, value)) => {
                map.insert(key, value);
            },
            Err(error) => return Err(format!("Failed to parse named arg: {:?}", error)),
        }
    }

    Ok(map)
}
