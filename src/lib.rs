use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufWriter, BufReader, self};

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
        "help" => show_help(command.other_args)?,
        "createList" => create_list()?,
        "list" => read_list()?,
        "deleteList" => delete_list()?,
        "add" => add_entry(command.other_args)?,
        "del" => delete_entry(command.other_args[0].parse().unwrap())?,
        _ => return Err("Invalid command".into()),
    }
    Ok(())
}

fn show_help(help_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if help_args.len() < 1 {
        println!("Usage: todo help <command_name>");
        println!("Type 'todo help commands' to view help for all supported commands");
        return Ok(());
    }
    //handle help for each command
    match help_args[0].as_str() {
        "commands" => {
            println!("Supported commands:\ncreateList, list, deleteList, add, del");
            println!("For more info use: todo help <command_name>")
        },
        "createList" => {
            println!("Command: todo createList\nDescription: Creates a todo.md if one is not present.")
        },
        "list" => {
            println!("Command: todo list\nDescription: Shows all TODO's present in the todo.md.")
        }
        "deleteList" => {
            println!("Command: todo deleteList\nDescription: Deletes the todo.md if one is present.")
        },
        "add" => {
            println!("Command: todo add <Text>\nDescription: Adds a new TODO with the text you enter.")
        },
        "del" => {
            println!(
                "Command: todo del <entry_number>\n\
                Description: Deletes a TODO entry at the indexed value in the list.\n\
                Use 'todo list' to view the list along with their indexes.")
        },
        _ => println!("help for command '{}' does not exist", help_args[0]),
    }
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
    let f = File::open("todo.md")?;
    let reader = BufReader::new(f);
    let file_lines = reader.lines();
    let mut line_num = 0;
    println!("TODO:");
    for line in file_lines {
        line_num += 1;
        println!("{}. {}", line_num, line.unwrap());
    }
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

fn delete_entry(entry_val: i32) -> Result<(), Box<dyn Error>> {
    let f = File::open("todo.md")?;
    let reader = BufReader::new(f);
    let mut file_lines = reader.lines();

    let mut keep_lines: Vec<String> = Vec::new();
    let mut line_num = 1;

    while line_num < entry_val {
        let cur_line = file_lines.next().unwrap()?;
        keep_lines.push(cur_line);
        line_num += 1;
    }
    for line in file_lines {
        let keep = line.unwrap();
        if line_num == entry_val {
            println!("Confirm deleting TODO: {}", keep);
            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation)?;
            if confirmation == "y\n" {
                println!("Deleted TODO");
                line_num += 1;
                continue;
            }
        }
        keep_lines.push(keep);
        line_num += 1;
    }
    let mut writer = BufWriter::new(File::create("todo.md")?);
    for line in keep_lines {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}
