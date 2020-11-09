# sqlast

Query parsing and composing via the [sqlparser](https://docs.rs/sqlparser/0.6.1/sqlparser/) library.

## Compiling

With a rust environment setup, `cargo build --release` and pull the binary from `target/release/sqlast`.

To cross compile architecture, OS, or libc use the `--target` opt.

## Usage

`sqlast parse 'SELECT * FROM abc'`

`sqlast compose '<json ast>'`

The AST format is a serialized form of the libraries [internal AST](https://docs.rs/sqlparser/0.6.1/sqlparser/ast/index.html).
