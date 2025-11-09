use doom_2038::doom;
use std::env;

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
