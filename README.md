# WIP(unusable) Another CLJ Language Server (ACLS)

You might be wondering, why another LSP server if we already have [clojure-lsp](https://github.com/clojure-lsp/clojure-lsp)?

That's because ACLS makes no effort to lint, or analyze your code, but rather focus on useful and out of the curve
functionality, such as:

- Standardization of namespace aliases,
- Move functions and modules,

These decisions make ACLS heavily opinionated, and you must use a LSP client that makes possible to use multiple LSP
servers, because ACLS probably won't be your primary LSP Server (may change in the future).

Heavily inspired by the great [rust-analyzer](https://github.com/rust-lang/rust-analyzer).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
