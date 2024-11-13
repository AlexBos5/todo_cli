use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: todo <command> [arguments...]");
        return;
    }
    let command = &args[1];
    let params: Vec<String> = args[2..].to_vec();
    let todo_contents =
        fs::read_to_string("/home/alex/repos/github/todoCLI/todo.data").expect("Can Read File");
    println!("Contents:\n{todo_contents}")
}
