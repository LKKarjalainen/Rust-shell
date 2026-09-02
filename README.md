# rust-shell

A small POSIX-style shell written in Rust, originally built as a solution to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

It reads commands from stdin at a `$ ` prompt, handles a few builtins itself
(`echo`, `type`, `pwd`, `cd`, `exit`), and looks up anything else on `PATH` and
executes it.

## Requirements

- Rust 1.96 or newer.
- A Unix-like OS.

## Running in dev mode

From the repository root:

```sh
cargo run
```
This builds an unoptimized binary with debug assertions and drops you straight
into the shell:

Go ahead and try these
```
$ echo hello
hello
$ type pwd
pwd is a shell builtin
$ exit
```

## Building for release

```sh
cargo build --release
```

The optimized binary is written to `target/release/rust-shell`.

## Running the release build

```sh
./target/release/rust-shell
```

To run it from anywhere, copy the binary onto your `PATH`:

```sh
cp target/release/rust-shell ~/.local/bin/
rust-shell
```

Or install it through Cargo, which builds in release mode and places the binary
in `~/.cargo/bin`:

```sh
cargo install --path .
```
