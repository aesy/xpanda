# Xpanda Lib

### [API Reference](https://aesy.github.io/xpanda/xpanda)

## Usage

First create a new `Xpanda` struct using the builder: 

```rust
use xpanda::Xpanda;

let xpanda = Xpanda::builder()
    // ...
    .build();
```

or use the default implementation:

```rust
let xpanda = Xpanda::default();
```

The default implementation sources values from environment variables and ignores unset variables (leaving an 
empty string).

The `Xpanda` struct implements a single method, `expand`, which will return a copy of the given string expanded 
according to the [pattern rules](../docs/PATTERNS.md). For example:

```rust
assert_eq!(xpanda.expand("${1:-default}"), Ok(String::from("default")));
```

The [API Reference](https://aesy.github.io/xpanda/xpanda) provides more details.

## Installation

Add `xpanda` manually as a dependency in your `Cargo.toml` file or use the cargo add command: 

```sh
cargo add xpanda
```

## MSRV

The Minimum Supported Rust Version is currently `1.63`.
