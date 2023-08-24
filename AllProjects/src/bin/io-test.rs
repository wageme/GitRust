/*
Type in "cargo run [args]" in the terminal
cargo run --bin io-test the "C:\Users\Wage Me\Desktop\rust\GitRust\AllProjects\src\bin\poem.txt"
Just running this program won't work
*/

use std::env; // environment variables
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}" , query);
    println!("In file {}", file_path);


    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");


}
