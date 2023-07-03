/*
Type in "cargo run [args]" in the terminal
cargo run the "C:\Users\Wage Me\Desktop\rust\GitRust\minigrep\src\poem.txt"
C:\Users\Wage Me\Desktop\rust\GitRust\minigrep
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




/*fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}*/