# Xpanda CLI

## Usage

CLI options and examples can be viewed via the CLI's help command:

```shell
xpanda --help
```

```
Unix shell-like parameter expansion/variable substitution.

This program will process text from a file or standard input and copy it to
standard output or a file with all variables expanded/substituted using the
provided variables.

Variables can appear in the input in any of the following forms:
$VAR                   substituted with the corresponding value for `VAR` if set, otherwise ` `.
${VAR}                 substituted with the corresponding value for `VAR` if set, otherwise ` `.
${VAR-default}         substituted with the corresponding value for `VAR` if set, otherwise `default`.
${VAR:-default}        substituted with the corresponding value for `VAR` if set and non-empty, otherwise `default`.
${VAR+alternative}     substituted with `alternative` if the corresponding value for `VAR` is set, otherwise ` `.
${VAR:+alternative}    substituted with `alternative` if the corresponding value for `VAR` is set and non-empty, otherwise ` `.
${VAR?}                substituted with the corresponding value for `VAR` if set, otherwise causes the program to exit with a status code of 1 and an error printed to standard error.
${VAR?error}           substituted with the corresponding value for `VAR` if set, otherwise causes the program to exit with a status code of 1 and `error` printed to standard error.
${VAR:?}               substituted with the corresponding value for `VAR` if set and non-empty, otherwise causes the program to exit with a status code of 1 and an error printed to standard error.
${VAR:?error}          substituted with the corresponding value for `VAR` if set and non-empty, otherwise causes the program to exit with a status code of 1 and `error` printed to standard error.
${#VAR}                substituted with the length of the corresponding value for `VAR` if set, otherwise `0`.
${#}                   substituted with a number of positional variables.
${!VAR}                substituted with the value of the variable named by the value of `VAR`.
${VAR:offset}          substituted with the value of `VAR` from character index `offset` to the end, otherwise ` `. A negative `offset` must be written as `${VAR: -1}` or `${VAR:(-1)}`.
${VAR:offset:length}   substituted with `length` characters of `VAR` from character index `offset`, otherwise ` `.
${VAR^}                substituted with the corresponding value for `VAR`, with the first character uppercased.
${VAR^^}               substituted with the corresponding value for `VAR`, with all characters uppercased.
${VAR,}                substituted with the corresponding value for `VAR`, with the first character lowercased.
${VAR,,}               substituted with the corresponding value for `VAR`, with all characters lowercased.
${VAR~}                substituted with the corresponding value for `VAR`, with the casing of the first character reversed.
${VAR~~}               substituted with the corresponding value for `VAR`, with the casing of all characters reversed.
${VAR#pattern}         substituted with the value of `VAR` with the shortest prefix matching the glob `pattern` removed.
${VAR##pattern}        substituted with the value of `VAR` with the longest prefix matching the glob `pattern` removed.
${VAR%pattern}         substituted with the value of `VAR` with the shortest suffix matching the glob `pattern` removed.
${VAR%%pattern}        substituted with the value of `VAR` with the longest suffix matching the glob `pattern` removed.
${VAR/pattern/repl}    substituted with the value of `VAR` with the first match of the glob `pattern` replaced with `repl`.
${VAR//pattern/repl}   substituted with the value of `VAR` with all matches of the glob `pattern` replaced with `repl`.

`VAR` above is a named variable. Positional variables are also supported and
are passed as trailing arguments to the program (see the examples). They can
be referenced using their index (starting at 1) - for example, `$1`
references the first positional variable, `$2` the second and so on. `$0` is
a space-concatenated string of all positional variables.

The `$` character is assumed to be the start of a variable. If the variable
does not match any of the forms listed above, the program will fail to parse
the variable and exit the program with a status code of 1.

EXAMPLES:
`echo '$VAR' | xpanda -v VAR=value`   substitute `$VAR` with `value` using a named variable argument.
`echo '$VAR' | VAR=value xpanda`      substitute `$VAR` with `value` using an environment variable.
`echo '$1' | xpanda -- value`         substitute `$1` with `value` using a positional variable argument.
`xpanda < some_file`                  output a copy of `some_file` with variables substituted with environment variables.
`xpanda -f var_file < some_file`      output a copy of `some_file` with variables substituted with variables from `var_file`.
`xpanda -v VAR=value < some_file`     output a copy of `some_file` with `$VAR` substituted with `value` using `-v`.

The given input must be ASCII or UTF-8 encoded and is read in full before any
output is written. Output is UTF-8 encoded. If no variables are provided, then
values are sourced from environment variables.

Usage: xpanda-cli [OPTIONS] [-- [POSITIONAL_VARS]...]

Arguments:
  [POSITIONAL_VARS]...
          Zero or more positional variable values. The first value can be
          referenced using `$1`, the second `$2` and so on.

          If any positional variables are provided then the default setting to
          source values from environment variables will be overridden. To
          continue sourcing from environment values as well, add the
          `--env-vars` flag.

Options:
  -u, --no-unset
          With this flag set, missing variables without any default value will
          cause the program to exit with a status code of 1. Off by default.

  -f, --var-file <FILE>
          Provide a file to source variable values from.

          This option can be used multiple times in order to add multiple files.

          Using this option will override the default setting to source values
          from environment variables. To continue sourcing from environment
          values as well, add the `--env-vars` flag.

          The file must be formatted as key=value pairs with one variable per
          line. Failure to parse this file will cause the program to exit with
          status code 1.

          Example:
          KEY1=value
          KEY2=value

  -e, --env-vars[=<ENV_VARS>]
          With this flag set, named variables will be sourced from environment
          variables in addition to any other provided variables. Named
          variables will always take precedence over environment variables
          though. This flag is implicitly true if no other variables
          are provided.

          [possible values: true, false]

  -v, --var <VAR>
          Adds a named variable to source from. The value should be a key value
          pair separated by a `=`, e.g. `-v NAME=value`.

          This option can be used multiple times in order to add multiple
          variables.

          Using this option will override the default setting to source values
          from environment variables. To continue sourcing from environment
          values as well, add the `--env-vars` flag.

  -i, --input <FILE>
          Provide a path to read from. This overrides the default behaviour of
          reading from standard input.

  -o, --output <FILE>
          Provide a path to write to. This overrides the default behaviour of
          writing to standard output. A new file is created if it doesn't
          already exists. Output is appended to it if it already exists.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Installation:

### Binaries

The latest stable `Xpanda` releases are available as [prebuilt binaries for 64-bit Linux and Windows under Github
releases](https://github.com/aesy/xpanda/releases). Binaries for other platforms have to be built from source.

#### Linux

Example on how to download and install an `Xpanda` binary using [curl](https://curl.se/):

```shell
curl -L https://github.com/aesy/xpanda/releases/download/v0.2.0/xpanda-linux-amd64 -o xpanda
chmod +x xpanda
sudo mv xpanda /usr/local/bin
```

#### Windows

Example on how to download an `Xpanda` binary using [curl](https://curl.se/):

```shell
curl -L https://github.com/aesy/xpanda/releases/download/v0.2.0/xpanda-windows-amd64.exe -o xpanda.exe
```

#### Build from source

To build the CLI, run the following command:

```shell
cargo build --bin xpanda-cli --target <target> --release
```

The resulting binary can be found in the repositories root directory under `target/<target>/release/` called
`xpanda-cli`.

The `<target>` options can be found
at [Rust's platform support page](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
