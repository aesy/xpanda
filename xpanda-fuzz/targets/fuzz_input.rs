#![no_main]

use libfuzzer_sys::fuzz_target;
use xpanda::Xpanda;

fuzz_target!(|data: &[u8]| {
    let xpanda = Xpanda::default();

    if let Ok(input) = std::str::from_utf8(data) {
        let _ = xpanda.expand(input);
    }
});
