# dotenv Specification

## History

The dotenv format originated from Heroku's [Twelve-Factor App](https://12factor.net/config) methodology (2012), which advocates storing configuration in environment variables. The `.env` file allows developers to simulate production environment variables locally.

The first implementation was [bkeepers/dotenv](https://github.com/bkeepers/dotenv) for Ruby (2013). Later, motdotla rewrote it in JavaScript as [motdotla/dotenv](https://github.com/motdotla/dotenv), which became the standard in the Node.js ecosystem and a widely ported reference implementation across languages.

## Current Status

dotenv has no official standard like an RFC. Implementation rules vary slightly between languages and tools, but the core syntax is largely consistent.

Since Node.js v20.6.0 (stable since v20.12.0), `--env-file` support has been built in, with the parser implemented in C++ (`src/node_dotenv.cc`). Its behavior differs slightly from npm dotenv.

## Specs in This Directory

| File | Description |
|------|-------------|
| [`dotenv`](dotenv) | Language specification. Adds variable substitution (`$VAR` / `${VAR:-default}`) and multi-line values to the basic dotenv format. Key charset follows POSIX shell convention (`[A-Za-z_][A-Za-z0-9_]*`). Semantics follow POSIX shell for quoting (expansion in double quotes, literal in single quotes); backtick-quoted values are literal (unlike POSIX shell). |
| [`dotenv-node.abnf`](dotenv-node.abnf) | [npm dotenv](https://github.com/motdotla/dotenv) (motdotla/dotenv v16.x) implementation spec. Based on per-line regex matching, no variable substitution. Appendix compares differences with Node.js built-in `parseEnv`. |
| [`README.md`](README.md) | This file. |

### Key Reference Implementations

- **npm dotenv**: [motdotla/dotenv](https://github.com/motdotla/dotenv)
- **dotenv-expand**: [motdotla/dotenv-expand](https://github.com/motdotla/dotenv-expand)
- **Node.js built-in**: `process.loadEnvFile()` / `--env-file`, parser at [src/node_dotenv.cc](https://github.com/nodejs/node/blob/main/src/node_dotenv.cc)
- **Python**: [theskumar/python-dotenv](https://github.com/theskumar/python-dotenv)
- **Ruby**: [bkeepers/dotenv](https://github.com/bkeepers/dotenv)
