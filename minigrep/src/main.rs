/*
Type in "cargo run [args]" in the terminal
cargo run the "C:\Users\Wage Me\Desktop\rust\GitRust\minigrep\src\poem.txt"
C:\Users\Wage Me\Desktop\rust\GitRust\minigrep


RESUME HERE
https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html
*/

use std::env; // environment variables
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);


    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}" , config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}
