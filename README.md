# sqlast

Query parsing and composing via the [sqlparser](https://docs.rs/sqlparser/0.6.1/sqlparser/) library.

This is a minimal wrapper to interact with the library, it has no buisness logic or rewriting code.

## Install

Download the latest release for either Linux or macOS [here](https://github.com/mobikitinc/sqlast/releases).

Put this file in your $PATH (this could be `~/bin`, `/usr/local/bin`, or another location). If you don't download the binary named `sqlast` (the linux default binary) be sure to rename it in your path.

## Compiling

With a rust environment setup, `cargo build --release` and pull the binary from `target/release/sqlast`.
The easist way to do this is by using a rust container image (either docker or Microsoft devcontainer images).

To cross compile for an architecture, OS, or libc distribution add the `--target` flag. This doesn't always work for every platform, so the best option is to nativly compile for an OS.

## Packaging with RPM

1. Open a VM/Container with CentOS
    * Match the CentOS version with the prod image type (or Amazon equivalent)
2. [Install Rust](https://www.rust-lang.org)
3. [Install cargo-rpm](https://www.rust-lang.org)
    * `cargo install cargo-rpm`
    * This will build the SPEC files for us
4. Run `cargo rpm init`
    * This will build a specfile in `.rpm`
5. Run `cargo rpm build`
    * Output files will be in `target/release/rpmbuild`

## Usage

`sqlast parse 'SELECT * FROM abc'`

`sqlast compose '<json ast>'`

The AST format is a serialized form of the libraries [internal AST](https://docs.rs/sqlparser/0.6.1/sqlparser/ast/index.html).

Errors will have a non-zero return code.
