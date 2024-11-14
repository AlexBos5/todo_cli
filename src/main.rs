use std::env;
use std::process;
use todo_cli::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Keyword: {}", command.keyword);
    println!("Other_args: {:?}", command.other_args);
    if let Err(e) = todo_cli::run(command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
