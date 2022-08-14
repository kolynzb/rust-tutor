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

- rust (rls) by rust-lang for linting
- better TOML by bung... for toml syntax highlighting.

# folder structure

- `Cargo.toml` - is pretty much like package.json for node.
- `src` is where you write all your rust code.

- To compile and run the cargo project use.
  `cargo run `
- To build the project.
  `cargo build`
- If your building for production. add the `--release` flag.

- Oh btw rust files cant start with a number as a name so `1print.rs` wont work.

# Contents

- [Print Statement & Formatting](/docs/0x1print.md).
- [Variables In Rust](/docs/0x2vars.md)
