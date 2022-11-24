# dotenvs

[![crates.io](https://img.shields.io/crates/v/dotenvs.svg)](https://crates.io/crates/dotenvs)
[![Released API docs](https://docs.rs/dotenvs/badge.svg)](https://docs.rs/dotenvs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

A correct dotenv library, which supports:

- Multiline values
- Variable substitution

## Parsing rules

- `BASIC=basic`
- empty lines are skipped
- lines beginning with `#` are treated as comments
- `#` marks the beginning of a comment (unless when the value is wrapped in quotes)
- empty values become empty strings
- inner quotes are maintained
- whitespace is removed from both ends of unquoted values
- single and double quoted values are escaped
- single and double quoted values maintain whitespace from both ends
- double quoted values expand new lines (`MULTILINE="new\nline"` becomes
  ```
  MULTILINE: "new
  line"
  ```

## Expanding rules

- `$KEY` will expand any env with the name `KEY`
- `${KEY}` will expand any env with the name `KEY`
- `\$KEY` will escape the `$KEY` rather than expand
- `${KEY:-default}` will first attempt to expand any env with the name `KEY`. If not one, then it will return `default`

## Usage

The easiest and most common usage consists on calling `load` when the
application starts, which will load environment variables from a file named
`.env` in the current directory or any of its parents.

If you need more control about the file name or its location, you can
use the `from_filename`, `from_path` or `from_read`.

## Examples

A `.env` file looks like this:

```dotenv
# a comment, will be ignored
REDIS_ADDRESS=localhost:6379
MEANING_OF_LIFE=42
```

You can optionally prefix each line with the word `export`, which will
conveniently allow you to source the whole file on your shell.

A sample project using dotenv would look like this:

```rust
fn main() {
    for (key, value) in dotenv::vars() {
        println!("{}: {}", key, value);
    }
}
```

## LICENSE

[MIT](LICENSE.md)
