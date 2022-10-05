<img height="140" src="./img/icon.png">

[![Crate][crate-image]][crate-url]
[![Downloads][downloads-image]][downloads-url]
[![Documentation][docs-image]][docs-url]
[![Build Status][github-actions-image]][github-actions-url]
[![Test coverage][codecov-image]][codecov-url]
[![MIT license][license-image]][license-url]

[crate-image]: https://img.shields.io/crates/v/xpanda?style=flat-square

[crate-url]: https://crates.io/crates/xpanda

[downloads-image]: https://img.shields.io/crates/d/xpanda?style=flat-square

[downloads-url]: https://crates.io/crates/xpanda

[docs-image]: https://img.shields.io/docsrs/xpanda?style=flat-square

[docs-url]: https://docs.rs/xpanda

[github-actions-image]: https://img.shields.io/github/workflow/status/aesy/xpanda/Continuous%20Integration?style=flat-square

[github-actions-url]: https://github.com/aesy/xpanda/actions

[codecov-image]: https://img.shields.io/codecov/c/github/aesy/xpanda?style=flat-square

[codecov-url]: https://codecov.io/github/aesy/xpanda

[license-image]: https://img.shields.io/github/license/aesy/xpanda?style=flat-square

[license-url]: https://github.com/aesy/xpanda/blob/master/LICENSE

Safe Unix shell-like parameter expansion/variable substitution for those who need a more powerful alternative to 
[`envsubst`](https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html) but don't want to resort 
to using Bash's [`eval`](https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html), while also not
wanting to use a full-on templating engine. `Xpanda` is available as a native single-binary Windows/Linux CLI tool and 
a Rust library.

[See how it compares to other programs](./docs/COMPARISON.md).

## Usage

Check out which [patterns](./docs/PATTERNS.md) `Xpanda` will recognize and expand. 

Also check out the readme of the submodule that you are interested in:

* [CLI](./xpanda-cli/README.md) 
* [LIB](./xpanda/README.md)

## Development

#### Prerequisites

* [Rust Stable 1.60.0+](https://www.rust-lang.org/tools/install)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Build

To compile the project, simply issue the following command:

```sh
$ cargo build
```

#### Test

##### Linting

This project uses [rustfmt](https://github.com/rust-lang/rustfmt) for formatting and 
[clippy](https://github.com/rust-lang/rust-clippy) for linting. Run them with:

```sh
$ cargo fmt 
$ cargo clippy
```

##### Unit/Integration/Doc Testing 

All code that goes into master must pass all tests. To run all tests, use:

```sh
$ cargo test
```

##### Benchmarking 

Benchmarks can be ran using:

```sh
$ cargo bench
```

##### Fuzz Testing 

And fuzz testing, located in the `xpanda-fuzz` directory, can be ran using the 
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) tool, which has to be installed first:

```sh
$ cargo install cargo-fuzz 
```

It must then be ran using the [nightly channel](https://rust-lang.github.io/rustup/concepts/channels.html):  

```sh
$ cargo +nightly fuzz run fuzz_input --fuzz-dir xpanda-fuzz
```

## Contribute

Use the [issue tracker](https://github.com/aesy/xpanda/issues) to report bugs or make feature requests. Pull requests 
are welcome, but it may be a good idea to create an issue to discuss any changes beforehand.

## License

MIT, see [LICENSE](/LICENSE) file.
