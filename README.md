# 2038 Doom Countdown
# FIX: WHY IS EVERYTHING u64!?!?!
[![Crates.io](https://img.shields.io/crates/v/doom-2038.svg)](https://crates.io/crates/doom-2038)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/doom-2038)
[![Downloads](https://img.shields.io/crates/d/doom-2038.svg)](https://crates.io/crates/doom-2038)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-lightgrey.svg)](https://opensource.org/licenses/Unlicense)

[Crate]: https://crates.io/crates/doom-2038
[Documentation]: https://docs.rs/doom-2038
[Downloads]: https://crates.io/crates/doom-2038
[License]: https://opensource.org/licenses/Unlicense


A Rust CLI application that counts down to the Year 2038 problem, which is associated with the maximum value for a 32-bit signed integer.

## Installation

To add the crate to your project, run:

```bash
cargo add doom-2038
```

Then, include it in your `main.rs`:

```rust
use doom_2038::{doom, DOOM_TS, time_left}; // and other functions/constants
```

### Usage

#### Running the Application

- When you run `main.rs` normally, it executes `doom(false)` to display the remaining time.

- To run the countdown continuously, provide one of the following arguments:

```bash
cargo run -- [c|count|countdown]
```

For example:

```bash
cargo run -- countdown
```

## Example

Here’s a simple example of how to use the library in your `main.rs`:

```rust
use std::env;
use doom_2038::{doom, time_left};

fn main() {
    let args: Vec<String> = env::args().collect();

    let countdown = match args.get(1).map(String::as_str) {
        None => false,
        Some("c") | Some("count") | Some("countdown") => true,
        Some(arg) => {
            eprintln!("Incorrect argument: {}", arg);
            eprintln!("Usage: cargo run -- [c|count|countdown]");
            return;
        }
    };

    doom(countdown);
}
```

## Enjoy!

Thank you for using the 2038 Doom Countdown crate! If you have any questions or suggestions, feel free to reach out or contribute to the project.
Please ⭐ my github!

