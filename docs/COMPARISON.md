# Comparison

This document lists which patterns from other programs that are supported/unsupported by `Xpanda`.

## GNU envsubst

| Pattern    | Description                                                |        Status |
|------------|:-----------------------------------------------------------|--------------:|
| `$param`   | `$param` if set, else empty                                |     SUPPORTED |
| `${param}` | `$param` if set and non-empty, else empty                  |     SUPPORTED |

## POSIX shell & bash

| Pattern                  | Description                                                |        Status |
|--------------------------|:-----------------------------------------------------------|--------------:|
| `$param`                 | `$param` if set, else empty                                |     SUPPORTED |
| `${param}`               | `$param` if set and non-empty, else empty                  |     SUPPORTED |
| `${param-pattern}`       | `$param` if set, else `pattern`                            |     SUPPORTED |
| `${param:-pattern}`      | `$param` if set and non-empty, else `pattern`              |     SUPPORTED |
| `${param=pattern}`       | `$param` and assign `pattern` to it if not set             | NOT SUPPORTED |
| `${param=-pattern}`      | `$param` and assign `pattern to it if not set or empty     | NOT SUPPORTED |
| `${param+pattern}`       | `pattern` if `$param` is set and non-empty, else empty     |     SUPPORTED |
| `${param:+pattern}`      | `pattern` if `$param` is set, else empty                   |     SUPPORTED |
| `${param?text}`          | `$param` if set, else exit with error `text`               |     SUPPORTED |
| `${param:?text}`         | `$param` if set and non-empty, else exit with error `text` |     SUPPORTED |
| `${#param}`              | Character length of `$param` if set, else `0`              |     SUPPORTED |
| `${param#text}`          | `$param` with start trimmed of `text` (lazy)               | NOT SUPPORTED |
| `${param##text}`         | `$param` with start trimmed of `text` (greedy)             | NOT SUPPORTED |
| `${param%text}`          | `$param` with end trimmed of `text` (lazy)                 | NOT SUPPORTED |
| `${param%%text}`         | `$param` with end trimmed of `text` (greedy)               | NOT SUPPORTED |
| `${param:offset}`        |                                                            | NOT SUPPORTED |
| `${param:offset:length}` |                                                            | NOT SUPPORTED |
| `${!param}`              |                                                            | NOT SUPPORTED |
