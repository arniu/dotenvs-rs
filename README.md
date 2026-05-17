# dotenvs

[![crates.io](https://img.shields.io/crates/v/dotenvs.svg)](https://crates.io/crates/dotenvs)
[![Released API docs](https://docs.rs/dotenvs/badge.svg)](https://docs.rs/dotenvs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

A correct dotenv library with variable substitution.

- Variable substitution: `$VAR`, `${VAR}`, `${VAR:-default}`
- Three quote styles (`' " ``), multi-line values
- Overwrite or preserve existing environment variables
- `export` prefix for shell compatibility

## Usage

> **Note**:
>
> The crate is named `dotenvs`, but its lib is `dotenv`.

```toml
[dependencies]
dotenvs = "0.2"
```

```rust
dotenv::load();

// iterate all variables
for (key, value) in dotenv::vars() {
    println!("{}: {}", key, value);
}
```

## License

[MIT](LICENSE.md)
