use std::env; // environment variables
use std::process;
use better_grep::Config;

// cargo run to ./src/poem.txt

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = better_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
