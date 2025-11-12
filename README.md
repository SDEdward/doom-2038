# 2038 Doom Countdown

A Rust CLI application that counts down to the Year 2038 problem, which is associated with the maximum value for a 32-bit signed integer.

## Installation

To add the crate to your project, run:

```bash
cargo add doom-2038
```

Then, include it in your `main.rs`:

```rust
use doom_2038::{doom, DOOM_TS, time_left};
```

## Features

### Simple Documentation

- **`fn doom(bool)`**
  - If the boolean is `true`, it continuously counts down to the Year 2038 problem.
  - If `false`, it displays the time left only once in a human-readable format.

- **`fn time_left()`**
  - Returns the time left until the Year 2038 problem as a `Duration`.

- **`const DOOM_TS`**
  - Represents the 32-bit signed integer limit (2,147,483,647) as a `u64`. 

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

### Incorrect Arguments

If you provide an incorrect argument, an error message will be displayed, guiding you on the correct usage:

```
Incorrect argument: <arg>
Usage: cargo run -- [c|count|countdown]
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

