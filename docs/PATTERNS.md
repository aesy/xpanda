# Patterns

The following patterns/rules are supported:

| Pattern                  | Description                                                            |
|--------------------------|:-----------------------------------------------------------------------|
| `$param`                 | `$param` if set, else empty                                            |
| `${param}`               | `$param` if set and non-empty, else empty                              |
| `${param-pattern}`       | `$param` if set, else `pattern`                                        |
| `${param:-pattern}`      | `$param` if set and non-empty, else `pattern`                          |
| `${param+pattern}`       | `pattern` if `param` is set and non-empty, else empty                  |
| `${param:+pattern}`      | `pattern` if `param` is set, else empty                                |
| `${param?text}`          | `$param` if set, else exit with error `text`                           |
| `${param:?text}`         | `$param` if set and non-empty, else exit with error `text`             |
| `${#param}`              | Character length of `$param` if set, else `0`                          |
| `${#}`                   | Yields the number of positional variables/arguments                    |
| `${!param}`              | The value of `param` is evaluated as a parameter                       |
| `${param^}`              | `$param` with the first character uppercased if set, else empty        |
| `${param^^}`             | `$param` with all characters uppercased if set, else empty             |
| `${param,}`              | `$param` with the first character lowercased if set, else empty        |
| `${param,,}`             | `$param` with all characters lowercased if set, else empty             |
| `${param~}`              | `$param` with the first characters' case reversed if set, else empty   |
| `${param~~}`             | `$param` with all characters case reversed if set, else empty          |

## Examples

| Pattern               |        VAR unset |           VAR="" | VAR="example" |
|-----------------------|-----------------:|-----------------:|--------------:|
| `$VAR`                |              ` ` |              ` ` |     `example` |
| `${VAR}`              |              ` ` |              ` ` |     `example` |
| `${VAR-default}`      |        `default` |              ` ` |     `example` |
| `${VAR:-default}`     |        `default` |        `default` |     `example` |
| `${VAR+alternative}`  |              ` ` |    `alternative` | `alternative` |
| `${VAR:+alternative}` |              ` ` |              ` ` | `alternative` |
| `${VAR?message}`      | error: `message` |              ` ` |     `example` |
| `${VAR:?message}`     | error: `message` | error: `message` |     `example` |
| `${#VAR}`             |              `0` |              `0` |           `7` |
| `${!VAR}`             |              ` ` |              ` ` |    `$example` |
| `${VAR^}`             |              ` ` |              ` ` |     `Example` |
| `${VAR^^}`            |              ` ` |              ` ` |     `EXAMPLE` |
| `${VAR,}`             |              ` ` |              ` ` |     `example` |
| `${VAR,,}`            |              ` ` |              ` ` |     `example` |
| `${VAR~}`             |              ` ` |              ` ` |     `Example` |
| `${VAR~~}`            |              ` ` |              ` ` |     `EXAMPLE` |

With `-u` set (CLI) or `no_unset = true` (API), the following rules take precedence:

| Pattern      | VAR unset |
|--------------|----------:|
| `$VAR`       |     error |
| `${VAR}`     |     error |
| `${#VAR}`    |     error |
| `${!VAR}`    |     error |
| `${VAR^}`    |     error |
| `${VAR^^}`   |     error |
| `${VAR,}`    |     error |
| `${VAR,,}`   |     error |
| `${VAR~}`    |     error |
| `${VAR~~}`   |     error |

Default/Alternative values can also be patterns:

| Pattern                      | VAR unset |                VAR="" |         VAR="example" |
|------------------------------|----------:|----------------------:|----------------------:|
| `${VAR:-$DEF}`               |    `$DEF` |                   ` ` |             `example` |
| `${VAR+${ALT:-alternative}}` |       ` ` | `${ALT:-alternative}` | `${ALT:-alternative}` |

Error messages can be omitted in which case a default message will be used:

| Pattern    |                      VAR unset |                         VAR="" |
|------------|-------------------------------:|-------------------------------:|
| `${VAR?}`  |          error: `VAR is unset` |                            ` ` |
| `${VAR:?}` | error: `VAR is unset or empty` | error: `VAR is unset or empty` |

Note that writing `$VAR?` (without braces) is probably a mistake as the question mark is then not evaluated as part of the pattern.

## Escaping

Patterns can be escaped with a preceding `$`. 

| Input           |                             Output |
|-----------------|-----------------------------------:|
| `$${VAR}`       |                           `${VAR}` |
| `${VAR-$$text}` | The text `$text` if `VAR` is unset |
