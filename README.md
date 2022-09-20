# LEARN RUST WITH ME

- This repo contains all my exercise files for my rust basics so feel free to join me on my [rust](https://www.rust-lang.org/) learning journey.

![rust muscot](./docs/images/muscot.png)

## Rust installation.

- I used this [video](https://youtu.be/enk0o7eWNsc) as a guide to install Rust on windows. You can follow the [documentation](https://www.rust-lang.org/learn/get-started) as well.
- To start a new rust project run.
  `cargo new <project-name>`
  rustup - version manager
  rustc - compiler ( `rustc filename.rs` compiles to filename.exe then run ./filname in the terminal).
  rustup - packakage manager

# Extensions to install

- rust (rls) by rust-lang _for linting_
- better TOML by bung... _for toml syntax highlighting_.

# Folder structure

- `Cargo.toml` - is pretty much like package.json for node.
- `src` is where you write all your rust code.

# Important commands

- To compile and run the cargo project use.
  `cargo run `
- To build the project.
  `cargo build`
- If your building for production. add the `--release` flag.
- To go through documentation.
  `cargo doc --open`
- To update crates
  `cargo update`

- Oh btw rust files cant start with a number as a name so `1print.rs` wont work.

# Summaries From [Rust Book](./docs/rust-programming-language-steve-klabnik.pdf).

- [Chpt 2 - Guessing Game](/docs/rust_book/0x02-GuessingGame.md) with [code](/src/guessing_game.rs)

- [Chpt 3 - Common Programming Concepts In Rust](/docs/rust_book/0x03-Common_Programming_Concepts.md) with [code](/src/)

# Contents (More Specific)

- [Print Statement & Formatting](/docs/0x1print.md) with [code](/src/print.rs).
- [Variables In Rust](/docs/0x2vars.md) with [code](/src/vars.rs).
- [Datatypes In Rust](/docs/0x03data-types.md) with [code](/src/types.rs).
  - [Strings In Rust](/docs/0x04-Strings.md) with [code](/src/strings.rs).
- [Tuples in Rust](/docs/0x05-Tuples.md) with [code](/src/tuples.rs)
- [Arrays in Rust](/docs/0x06-Arrays.md) with [code](/src/arrays.rs)
- [Vectors in Rust](/docs/0x07-Vectors.md) with [code](/src/vectors.rs)
- [Functions in Rust](/docs/0x08-Functions.md).
- [Control Flow in Rust](/docs/0x09-Control_flow.md) with [code](/src/control_flow.rs.rs)

<!-- 52:49 -->
