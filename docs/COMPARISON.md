# Comparison

This document lists which patterns from other programs that are supported/unsupported by `Xpanda`.

## GNU envsubst

| Pattern    | Description                               |    Status |
|------------|:------------------------------------------|----------:|
| `$param`   | `$param` if set, else empty               | SUPPORTED |
| `${param}` | `$param` if set and non-empty, else empty | SUPPORTED |

## POSIX shell

Patterns defined by the POSIX parameter expansion specification. These are
portable to any conforming `/bin/sh`.

| Pattern             | Description                                                                   |        Status |
|---------------------|:------------------------------------------------------------------------------|--------------:|
| `${param-word}`     | yields the value of `param` if set, else `word`                               |     SUPPORTED |
| `${param:-word}`    | yields the value of `param` if set and non-empty, else `word`                 |     SUPPORTED |
| `${param=word}`     | yields the value of `param` and assigns `word` to it if not set               | NOT SUPPORTED |
| `${param:=word}`    | yields the value of `param` and assigns `word` to it if not set or empty      | NOT SUPPORTED |
| `${param+word}`     | yields `word` if `param` is set, else nothing                                 |     SUPPORTED |
| `${param:+word}`    | yields `word` if `param` is set and non-empty, else nothing                   |     SUPPORTED |
| `${param?word}`     | yields the value of `param` if set, else exit with error `word`               |     SUPPORTED |
| `${param:?word}`    | yields the value of `param` if set and non-empty, else exit with error `word` |     SUPPORTED |
| `${#param}`         | yields the length of `param` if set, else `0`                                 |     SUPPORTED |
| `${param#pattern}`  | yields the value of `param` with the shortest matching prefix removed         |     SUPPORTED |
| `${param##pattern}` | yields the value of `param` with the longest matching prefix removed          |     SUPPORTED |
| `${param%pattern}`  | yields the value of `param` with the shortest matching suffix removed         |     SUPPORTED |
| `${param%%pattern}` | yields the value of `param` with the longest matching suffix removed          |     SUPPORTED |

## Bash

Patterns added by bash on top of POSIX. Some are also present in ksh93 and zsh
but none are portable POSIX shell.

| Pattern                         | Description                                                                  |    Status |
|---------------------------------|:-----------------------------------------------------------------------------|----------:|
| `${#}`                          | yields the number of positional arguments (POSIX spells this `$#`)           | SUPPORTED |
| `${!param}`                     | yields the value of the value of `param` (indirect reference)                | SUPPORTED |
| `${param:offset}`               | yields the value of `param` from index `offset` to the end                   | SUPPORTED |
| `${param:offset:length}`        | yields the value of `param` from index `offset` for `length` characters      | SUPPORTED |
| `${param^}`                     | first letter of `param` uppercased                                           | SUPPORTED |
| `${param^^}`                    | all letters of `param` uppercased                                            | SUPPORTED |
| `${param,}`                     | first letter of `param` lowercased                                           | SUPPORTED |
| `${param,,}`                    | all letters of `param` lowercased                                            | SUPPORTED |
| `${param~}`                     | casing of the first letter of `param` reversed (bash 4; dropped in bash 5.1) | SUPPORTED |
| `${param~~}`                    | casing of all letters of `param` reversed (bash 4; dropped in bash 5.1)      | SUPPORTED |
| `${param/pattern/replacement}`  | first match of `pattern` in `param` replaced with `replacement`              | SUPPORTED |
| `${param//pattern/replacement}` | every match of `pattern` in `param` replaced with `replacement`              | SUPPORTED |

Arrays such as `$@` are not supported.

`$0` is equivalent with `$*`.
