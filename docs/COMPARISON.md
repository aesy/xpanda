# Comparison

This document lists which patterns from other programs that are supported/unsupported by `Xpanda`.

## GNU envsubst

| Pattern    | Description                                                |        Status |
|------------|:-----------------------------------------------------------|--------------:|
| `$param`   | `$param` if set, else empty                                |     SUPPORTED |
| `${param}` | `$param` if set and non-empty, else empty                  |     SUPPORTED |

## POSIX shell & bash

| Pattern                  | Description                                                                                                 |        Status |
|--------------------------|:------------------------------------------------------------------------------------------------------------|--------------:|
| `$param`                 | yields the value of `param` if set, else nothing                                                            |     SUPPORTED |
| `${param}`               | yields the value of `param` if set and non-empty, else nothing                                              |     SUPPORTED |
| `${param-word}`          | yields the value of `param` if set, else `word`                                                             |     SUPPORTED |
| `${param:-word}`         | yields the value of `param` if set and non-empty, else `word`                                               |     SUPPORTED |
| `${param=word}`          | yields the value of `param` and assign `word` to it if not set                                              | NOT SUPPORTED |
| `${param=-word}`         | yields the value of `param` and assign `word to it if not set or empty                                      | NOT SUPPORTED |
| `${param+word}`          | yields `word` if `param` is set and non-empty, else nothing                                                 |     SUPPORTED |
| `${param:+word}`         | yields `word` if `param` is set, else nothing                                                               |     SUPPORTED |
| `${param?word}`          | yields the value of `param` if set, else exit with error `word`                                             |     SUPPORTED |
| `${param:?word}`         | yields the value of `param` if set and non-empty, else exit with error `word`                               |     SUPPORTED |
| `${#param}`              | yields the length of `param` if set, else `0`                                                               |     SUPPORTED |
| `${#}`                   | yields the number of arguments                                                                              |     SUPPORTED |
| `${!param}`              | yields the value of the value of `param`                                                                    |     SUPPORTED |
| `${param#pattern}`       | yields the value of `param` with the start trimmed of `pattern` (lazy)                                      | NOT SUPPORTED |
| `${param##pattern}`      | yields the value of `param` with the start trimmed of `pattern` (greedy)                                    | NOT SUPPORTED |
| `${param%pattern}`       | yields the value of `param` with the end trimmed of `pattern` (lazy)                                        | NOT SUPPORTED |
| `${param%%pattern}`      | yields the value of `param` with the end trimmed of `pattern` (greedy)                                      | NOT SUPPORTED |
| `${param:offset}`        | yields the value of `param` from index `offset` to the end                                                  | NOT SUPPORTED |
| `${param:offset:length}` | yields the value of `param` from index `offset` to `offset` + `length`                                      | NOT SUPPORTED |
| `${param^}`              | yields the value of `param` with the first letter in uppercase if set and non-empty, else nothing           |     SUPPORTED |
| `${param^^}`             | yields the value of `param` in all uppercase if set and non-empty, else nothing                             |     SUPPORTED |
| `${param,}`              | yields the value of `param` with the first letter in lowercase if set and non-empty, else nothing           |     SUPPORTED |
| `${param,,}`             | yields the value of `param` in all lowercase if set and non-empty, else nothing                             |     SUPPORTED |
| `${param~}`              | yields the value of `param` with the casing of the first letter reversed if set and non-empty, else nothing |     SUPPORTED |
| `${param~~}`             | yields the value of `param` with the casing of all characters reversed if set and non-empty, else nothing   |     SUPPORTED |

Arrays such as `$@` are not supported.

`$0` is equivalent with `$*`.
