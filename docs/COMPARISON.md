# Comparison

This document lists which patterns from other programs that are supported/unsupported by xpanda.

## Envsubst

| Pattern        | Description                                                |        Status |
|----------------|:-----------------------------------------------------------|--------------:|
| $param         | `$param` if set, else empty                                |     SUPPORTED |
| ${param}       | `$param` if set and non-empty, else empty                  |     SUPPORTED |

## Bash

| Pattern                | Description                                                |        Status |
|------------------------|:-----------------------------------------------------------|--------------:|
| $param                 | `$param` if set, else empty                                |     SUPPORTED |
| ${param}               | `$param` if set and non-empty, else empty                  |     SUPPORTED |
| ${param-word}          | `$param` if set, else `word`                               |     SUPPORTED |
| ${param:-word}         | `$param` if set and non-empty, else `word`                 |     SUPPORTED |
| ${param=word}          | `$param` and assign `word` to it if not set                | NOT SUPPORTED |
| ${param=-word}         | `$param` and assign `word to it if not set or empty        | NOT SUPPORTED |
| ${param+word}          | `word` if `$param` is set, else `$param`                   |     SUPPORTED |
| ${param:+word}         | `word` if `$param` is set and non-empty, else `$param`     |     SUPPORTED |
| ${param?word}          | `$param` if set, else exit with error `word`               |     SUPPORTED |
| ${param:?word}         | `$param` if set and non-empty, else exit with error `word` |     SUPPORTED |
| ${#param}              | Character length of `$param` if set, else `0`              |     SUPPORTED |
| ${param#word}          | `$param` with start trimmed of `word` (lazy)               | NOT SUPPORTED |
| ${param##word}         | `$param` with start trimmed of `word` (greedy)             | NOT SUPPORTED |
| ${param%word}          | `$param` with end trimmed of `word` (lazy)                 | NOT SUPPORTED |
| ${param%%word}         | `$param` with end trimmed of `word` (greedy)               | NOT SUPPORTED |
| ${param:offset}        |                                                            | NOT SUPPORTED |
| ${param:offset:length} |                                                            | NOT SUPPORTED |
| ${!param}              |                                                            | NOT SUPPORTED |
