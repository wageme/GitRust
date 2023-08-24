/*
Type in "cargo run --bin minigrep [args] "C:\Users\Wage Me\Desktop\rust\GitRust\AllProjects\src\bin\poem.txt"" in the terminal

To search as normal, write:
cargo run the poem.txt

To search while ignoring case, write:
$Env:IGNORE_CASE=1; cargo run a "C:\Users\Wage Me\Desktop\rust\GitRust\AllProjects\src\bin\poem.txt"
C:\Users\Wage Me\Desktop\rust\GitRust\minigrep

To make ignore case persist in powershell, write:
Remove-Item Env:IGNORE_CASE

To redirect output, write:
cargo run to poem.txt > output.txt
*/

use std::env; // environment variables
use std::process;
use AllProjects::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = AllProjects::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
