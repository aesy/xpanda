#![no_main]

use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;
use std::str;
use xpanda::Xpanda;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = str::from_utf8(data) else {
        return;
    };
    let (value, template) = input.split_once('\0').unwrap_or(("a\u{e5}\u{e4}\u{f6}b", input));
    let named_vars = HashMap::from([
        (String::from("VAR"), String::from(value)),
        (String::from("REF"), String::from("VAR")),
        (String::from("EMPTY"), String::new()),
    ]);
    let xpanda = Xpanda::builder()
        .with_named_vars(named_vars)
        .with_positional_vars(vec![String::from(value), String::from("1")])
        .build();

    let _ = xpanda.expand(template);
});
