# Patterns

The following patterns/rules are supported:

| Pattern        | Description                                                |
|----------------|:-----------------------------------------------------------|
| $param         | `$param` if set, else empty                                |
| ${param}       | `$param` if set and non-empty, else empty                  |
| ${param-word}  | `$param` if set, else `word`                               |
| ${param:-word} | `$param` if set and non-empty, else `word`                 |
| ${param+word}  | `word` if `$param` is set, else `$param`                   |
| ${param:+word} | `word` if `$param` is set and non-empty, else `$param`     |
| ${param?word}  | `$param` if set, else exit with error `word`               |
| ${param:?word} | `$param` if set and non-empty, else exit with error `word` |
| ${#param}      | Character length of `$param` if set, else `0`              |

## Examples

| Pattern             |      VAR missing |           VAR="" | VAR="example" |
|---------------------|-----------------:|-----------------:|--------------:|
| $VAR                |              ` ` |              ` ` |     `example` |
| ${VAR}              |              ` ` |              ` ` |     `example` |
| ${VAR-default}      |        `default` |              ` ` |     `example` |
| ${VAR:-default}     |        `default` |        `default` |     `example` |
| ${VAR+alternative}  |              ` ` |    `alternative` | `alternative` |
| ${VAR:+alternative} |              ` ` |              ` ` | `alternative` |
| ${VAR?message}      | error: `message` |              ` ` |     `example` |
| ${VAR:?message}     | error: `message` | error: `message` |     `example` |
| ${#VAR}             |              `0` |              `0` |           `7` |

With `-u` set (CLI) or `no_unset = true` (API), the following rules take precedence:

| Pattern | VAR missing |
|---------|------------:|
| $VAR    |       error |
| ${VAR}  |       error |
| ${#VAR} |       error |

Default/Alternative values can also be patterns:

| Pattern                    | VAR missing |                VAR="" |         VAR="example" |
|----------------------------|------------:|----------------------:|----------------------:|
| ${VAR:-$DEF}               |      `$DEF` |                   ` ` |             `example` |
| ${VAR+${ALT:-alternative}} |         ` ` | `${ALT:-alternative}` | `${ALT:-alternative}` |

Error messages can be omitted in which case a default message will be used:

| Pattern  |          VAR missing |                VAR="" |
|----------|---------------------:|----------------------:|
| ${VAR?}  | error: `VAR not set` |                   ` ` |
| ${VAR:?} | error: `VAR not set` | error: `VAR is empty` |

Note that writing `$VAR?` (without braces) is probably a mistake as the question mark is then not evaluated as part of the pattern.
