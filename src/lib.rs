use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Command {
    pub keyword: String,
    pub other_args: Vec<String>,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let keyword = args[1].clone();
        let other_args = args[2..].to_vec();
        Ok(Command {
            keyword,
            other_args,
        })
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    match command.keyword.as_str() {
        "help" => show_help()?,
        "createList" => create_list()?,
        "list" => read_list()?,
        "deleteList" => delete_list()?,
        "add" => add_entry(command.other_args)?,
        "del" => delete_entry()?,
        _ => return Err("Invalid command".into()),
    }
    Ok(())
}

fn show_help() -> Result<(), Box<dyn Error>> {
    let current_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("todo.md");
    println!("The current directory is {:?}", current_path.display());
    let todo_contents = fs::read_to_string(&current_path)?;
    println!("Contents:\n{todo_contents}");
    Ok(())
}

fn create_list() -> Result<(), Box<dyn Error>> {
    File::create_new("todo.md")?;
    Ok(())
}

fn read_list() -> Result<(), Box<dyn Error>> {
    let current_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("todo.md");
    println!("The current directory is {:?}", current_path.display());
    let todo_contents = fs::read_to_string(&current_path)?;
    println!("Contents:\n{todo_contents}");
    Ok(())
}

fn delete_list() -> Result<(), Box<dyn Error>> {
    fs::remove_file("todo.md")?;
    Ok(())
}

fn add_entry(entry: Vec<String>) -> Result<(), Box<dyn Error>> {
    let current_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("todo.md");
    let mut f = File::options().append(true).open(&current_path)?;
    writeln!(&mut f, "{}", entry.join(" "))?;
    Ok(())
}

fn delete_entry() -> Result<(), Box<dyn Error>> {
    Ok(())
}
