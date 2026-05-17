# dotenv Specification

## History

The dotenv format originated from Heroku's [Twelve-Factor App](https://12factor.net/config) methodology (2012), which advocates storing configuration in environment variables. The `.env` file allows developers to simulate production environment variables locally.

The first implementation was [bkeepers/dotenv](https://github.com/bkeepers/dotenv) for Ruby (2013). Later, motdotla rewrote it in JavaScript as [motdotla/dotenv](https://github.com/motdotla/dotenv), which became the standard in the Node.js ecosystem and a widely ported reference implementation across languages.

## Current Status

dotenv has no official standard like an RFC. Implementation rules vary slightly between languages and tools, but the core syntax is largely consistent.

Since Node.js v20.6.0 (stable since v24.10.0, v22.21.0 LTS), `--env-file` support has been built in, with the parser implemented in C++ (`src/node_dotenv.cc`). Its behavior differs slightly from npm dotenv.

## Specs in This Directory

### [`dotenv.abnf`](dotenv.abnf)

Language specification. Extends the Node.js built-in format with variable substitution (`$VAR` / `${VAR:-default}`) and a restricted POSIX-compatible key charset (`[A-Za-z_][A-Za-z0-9_]*`). Semantics follow POSIX shell quoting rules (expansion in double quotes, literal in single quotes); backtick-quoted values are literal (unlike POSIX shell).

### [`dotenv-node.abnf`](dotenv-node.abnf)

Node.js built-in `parseEnv` / `--env-file` implementation spec. C++ parser in [`src/node_dotenv.cc`](https://github.com/nodejs/node/blob/main/src/node_dotenv.cc).

### Key Reference Implementations

- **npm dotenv**: [motdotla/dotenv](https://github.com/motdotla/dotenv)
- **dotenv-expand**: [motdotla/dotenv-expand](https://github.com/motdotla/dotenv-expand)
- **Node.js built-in**: `process.loadEnvFile()` / `--env-file`, parser at [src/node_dotenv.cc](https://github.com/nodejs/node/blob/main/src/node_dotenv.cc)
- **Python**: [theskumar/python-dotenv](https://github.com/theskumar/python-dotenv)
- **Ruby**: [bkeepers/dotenv](https://github.com/bkeepers/dotenv)
