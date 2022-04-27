# dotenvs

![CI](https://github.com/arniu/dotenvs-rs/workflows/CI/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/dotenv.svg)](https://crates.io/crates/dotenvs)

**Achtung!** This is a v0.\* version! Expect bugs and issues all around.
Submitting pull requests and issues is highly encouraged!

Quoting [bkeepers/dotenv][dotenv]:

> Storing [configuration in the environment](http://www.12factor.net/config)
> is one of the tenets of a [twelve-factor app](http://www.12factor.net/).
> Anything that is likely to change between deployment environments–such as
> resource handles for databases or credentials for external services–should
> be extracted from the code into environment variables.

This library is meant to be used on development or testing environments in
which setting environment variables is not practical. It loads environment
variables from a `.env` file, if available, and mashes those with the actual
environment variables provided by the operative system.

[dotenv]: https://github.com/bkeepers/dotenv

## Usage

The easiest and most common usage consists on calling `dotenv::dotenv` when the
application starts, which will load environment variables from a file named
`.env` in the current directory or any of its parents; after that, you can just call
the environment-related method you need as provided by `std::env`.

If you need finer control about the name of the file or its location, you can
use the `from_filename` and `from_path` methods provided by the crate.

## Examples

A `.env` file looks like this:

```sh
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

## Variable substitution

It's possible to reuse variables in the `.env` file using `$VARIABLE` syntax.
The syntax and rules are similar to bash ones, here's the example:

```sh

VAR=one
VAR_2=two

# Non-existing values are replaced with an empty string
RESULT=$NOPE #value: '' (empty string)

# All the letters after $ symbol are treated as the variable name to replace
RESULT=$VAR #value: 'one'

# Double quotes do not affect the substitution
RESULT="$VAR" #value: 'one'

# Different syntax, same result
RESULT=${VAR} #value: 'one'

# Curly braces are useful in cases when we need to use a variable with non-alphanumeric name
RESULT=$VAR_2 #value: 'one_2' since $ with no curly braces stops after first non-alphanumeric symbol
RESULT=${VAR_2} #value: 'two'

# The replacement can be escaped with either single quotes or a backslash:
RESULT='$VAR' #value: '$VAR'
RESULT=\$VAR #value: '$VAR'

# Environment variables are used in the substutution and always override the local variables
RESULT=$PATH #value: the contents of the $PATH environment variable
PATH="My local variable value"
RESULT=$PATH #value: the contents of the $PATH environment variable, even though the local variable is defined
```

Dotenv will parse the file, substituting the variables the way it's described in the comments.

## Support multiline value
