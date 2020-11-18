# sqlast

Query parsing and composing via the [sqlparser](https://docs.rs/sqlparser/0.6.1/sqlparser/) library.

This is a minimal wrapper to interact with the library, it has no buisness logic or rewriting code.

## Compiling

With a rust environment setup, `cargo build --release` and pull the binary from `target/release/sqlast`.
The easist way to do this is by using a rust container image (either docker or Microsoft devcontainer images).

To cross compile for an architecture, OS, or libc distribution add the `--target` flag. This doesn't always work for every platform, so the best option is to nativly compile for an OS.

## Usage

`sqlast parse 'SELECT * FROM abc'`

`sqlast compose '<json ast>'`

The AST format is a serialized form of the libraries [internal AST](https://docs.rs/sqlparser/0.6.1/sqlparser/ast/index.html).

Errors will have a non-zero return code.
