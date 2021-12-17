# lox_interpreter

This project is an implementation of a tree-walk interpreter of the Lox programming language. See the excellent [Crafting Interpreters](https://craftinginterpreters.com/) book for more details.

I've tried to re-implement the Java-based interpreter from the aforementioned book in (as much as possible) idiomatic Rust.

`lox_interpreter` is still a work in progress.

## Building and running

`lox_interpreter` is written in Rust, and can be built using Cargo. Once you have `rustc` and `Cargo` [installed](https://www.rust-lang.org/tools/install), clone this project and run `cargo build` from inside the project directory.

```
$ git clone https://github.com/tverghis/lox_interpreter.git
$ cd lox_interpreter
$ cargo build --release
```

Cargo places the executable under the `./target/release` folder.

### Usage

If the program is run with no arguments, it starts up a REPL that executes individual Lox statements:

```
$ ./target/release/lox_interpreter
```
You may also use this program to execute a Lox source file by passing the file path as the `--file`/`-f` argument:

```
$ ./target/release/lox_interpreter -f foo.lox
```
A help message can be printed with the `--help` flag.
