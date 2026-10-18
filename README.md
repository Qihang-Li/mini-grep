# minigrep

A small command-line text search tool written in Rust, based on the
I/O project in Chapter 12 of *The Rust Programming Language*.

This learning project practices argument parsing, file I/O, error
handling, lifetimes, iterators, and unit testing. It uses Rust 2024
and has no third-party dependencies.

## Usage

Run these examples from the project root in Bash or WSL.

Case-sensitive search:

```bash
env -u IGNORE_CASE cargo run --quiet --locked -- temper Sonnet_18.txt
```

Case-insensitive search:

```bash
IGNORE_CASE=1 cargo run --quiet --locked -- tEmPeR Sonnet_18.txt
```

Both examples print:

```text
Thou art more lovely and more temperate:
```

## Behavior

- Searches one UTF-8 file for a literal substring.
- Reads the entire file into memory.
- Prints matching lines to stdout in their original order and capitalization.
- Enables case-insensitive matching by lowercasing the query and each line.
- Setting `IGNORE_CASE=0` still enables case-insensitive matching;
  unset the variable to restore case-sensitive matching.
- Reports missing arguments and file-reading errors to stderr and exits with status 1.
- Exits with status 0 after a successful search, including when no lines match.

## Development checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked
cargo test --locked
```

The two unit tests cover case-sensitive and case-insensitive search.

## License

The Rust source code in this project is licensed under the [MIT License](LICENSE).

Portions are adapted from [The Rust Programming Language](https://github.com/rust-lang/book),
used under its MIT license.

Bundled sample texts are excluded from this license and retain their original copyright status.