# dotenvs

> This was extracted from <https://github.com/hoijui/dotenv>.

## Features

- Multiline values
- Variable substitution

## Usage

The easiest and most common usage consists on calling `dotenv::load` when the
application starts, which will load environment variables from a file named
`.env` in the current directory or any of its parents.

If you need more control about the file name or its location, you can
use the `load_filename` and `load_path`.

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
