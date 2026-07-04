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
| `${param:offset}`        | `$param` from index `offset` to the end if set, else empty             |
| `${param:offset:length}` | `$param` from index `offset` to `offset` + `length` if set, else empty |
| `${param^}`              | `$param` with the first character uppercased if set, else empty        |
| `${param^^}`             | `$param` with all characters uppercased if set, else empty             |
| `${param,}`              | `$param` with the first character lowercased if set, else empty        |
| `${param,,}`             | `$param` with all characters lowercased if set, else empty             |
| `${param~}`              | `$param` with the first characters' case reversed if set, else empty   |
| `${param~~}`             | `$param` with all characters case reversed if set, else empty          |
| `${param#pattern}`       | `$param` with the shortest matching prefix removed                     |
| `${param##pattern}`      | `$param` with the longest matching prefix removed                      |
| `${param%pattern}`       | `$param` with the shortest matching suffix removed                     |
| `${param%%pattern}`      | `$param` with the longest matching suffix removed                      |
| `${param/pattern/repl}`  | `$param` with the first match of `pattern` replaced with `repl`        |
| `${param//pattern/repl}` | `$param` with every match of `pattern` replaced with `repl`            |

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
| `${VAR:2}`            |              ` ` |              ` ` |       `ample` |
| `${VAR:2:4}`          |              ` ` |              ` ` |        `ampl` |
| `${VAR^}`             |              ` ` |              ` ` |     `Example` |
| `${VAR^^}`            |              ` ` |              ` ` |     `EXAMPLE` |
| `${VAR,}`             |              ` ` |              ` ` |     `example` |
| `${VAR,,}`            |              ` ` |              ` ` |     `example` |
| `${VAR~}`             |              ` ` |              ` ` |     `Example` |
| `${VAR~~}`            |              ` ` |              ` ` |     `EXAMPLE` |
| `${VAR#*e}`           |              ` ` |              ` ` |      `xample` |
| `${VAR##*e}`          |              ` ` |              ` ` |           ` ` |
| `${VAR%e*}`           |              ` ` |              ` ` |      `exampl` |
| `${VAR%%e*}`          |              ` ` |              ` ` |           ` ` |
| `${VAR/e/E}`          |              ` ` |              ` ` |     `Example` |
| `${VAR//e/E}`         |              ` ` |              ` ` |     `ExamplE` |

With `-u` set (CLI) or `no_unset = true` (API), the following rules take precedence:

| Pattern       | VAR unset |
|---------------|----------:|
| `$VAR`        |     error |
| `${VAR}`      |     error |
| `${#VAR}`     |     error |
| `${!VAR}`     |     error |
| `${VAR:2}`    |     error |
| `${VAR:2:4}`  |     error |
| `${VAR^}`     |     error |
| `${VAR^^}`    |     error |
| `${VAR,}`     |     error |
| `${VAR,,}`    |     error |
| `${VAR~}`     |     error |
| `${VAR~~}`    |     error |
| `${VAR#pat}`  |     error |
| `${VAR##pat}` |     error |
| `${VAR%pat}`  |     error |
| `${VAR%%pat}` |     error |
| `${VAR/a/b}`  |     error |
| `${VAR//a/b}` |     error |

Default/Alternative values can also be patterns:

| Pattern                      | VAR unset |                VAR="" |         VAR="example" |
|------------------------------|----------:|----------------------:|----------------------:|
| `${VAR:-$DEF}`               |    `$DEF` |                   ` ` |             `example` |
| `${VAR+${ALT:-alternative}}` |       ` ` | `${ALT:-alternative}` | `${ALT:-alternative}` |

Error messages can be omitted, in which case a default message will be used:

| Pattern    |                      VAR unset |                         VAR="" |
|------------|-------------------------------:|-------------------------------:|
| `${VAR?}`  |          error: `VAR is unset` |                            ` ` |
| `${VAR:?}` | error: `VAR is unset or empty` | error: `VAR is unset or empty` |

Note that writing `$VAR?` (without braces) is probably a mistake as the question mark is then not evaluated as part of
the pattern.

## Glob patterns

`#`, `##`, `%`, `%%`, `/` and `//` accept a glob pattern. Supported metacharacters:

| Pattern | Description                               |
|---------|-------------------------------------------|
| `*`     | matches any (possibly empty) sequence     |
| `?`     | matches exactly one character             |
| `[abc]` | matches any single character in the class |
| `[a-z]` | matches any single character in the range |
| `\*`    | backslash escapes the next metacharacter  |

## Substring offsets

The offset and length in `${param:offset}` / `${param:offset:length}` may
be surrounded by parentheses, which is necessary when the offset starts
with `-` to disambiguate from `${param:-default}`. Either form works:

| Form          |
|---------------|
| `${VAR:(-3)}` |
| `${VAR: -3}`  |

A space between `:` and `-` is also a valid disambiguator.

## Whitespace in braces

Whitespace inside `${ ... }` is significant only as data. The parser trims spaces right after the opening `{` and right
after an identifier that follows the opening `{`. So `${ VAR }` is the same as `${VAR}`, but `${VAR-  foo  }` defaults
to `"  foo  "`.

## Quoted patterns

Inside `${...}`, single (`'...'`) and double (`"..."`) quotes work like in
POSIX shell:

* The quote characters are not part of the value.
* `'...'` is fully literal - `$VAR` is **not** expanded.
* `"..."` expands `$VAR` inside, but otherwise the contents are literal.
* Quoted regions may contain characters that would otherwise terminate the brace expression: `}`, `:`, `-`, etc.
* In glob-pattern positions (`#`, `%`, `/`), quoted characters are matched literally - `${VAR#"*"}` removes a literal
  `*` prefix, not a glob match.

## Escaping

Patterns can be escaped with a preceding `$`.

| Input           |                             Output |
|-----------------|-----------------------------------:|
| `$${VAR}`       |                           `${VAR}` |
| `${VAR-$$text}` | The text `$text` if `VAR` is unset |
