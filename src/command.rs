use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::exit,
};

pub struct Id(pub String);

pub enum Command {
    Add(String),
    Get(Id),
    Update(Id, String),
    Delete(Id),
    List,
    Help,
}

fn get_file() -> PathBuf {
    let home: &str = Box::leak(std::env::var("HOME").unwrap().into_boxed_str());

    Path::new(home).join(".todo_txt")
}

fn list() {
    match fs::read_to_string(get_file()) {
        Ok(content) => {
            let todos = content.split("\n").collect::<Vec<&str>>();
            for (index, todo) in todos.iter().enumerate() {
                if todo.len() > 0 {
                    println!("{}: {}", index, todo);
                }
            }
        }
        Err(_) => println!("File does not exist!"),
    }
}

fn write_to_todos(input: String, append: bool) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .open(get_file())
        .unwrap();

    if let Err(e) = file.write_all(input.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn add(input: String) {
    write_to_todos(format!("\n{}", input), true)
}

fn get(Id(id): Id) {
    match fs::read_to_string(get_file()) {
        Ok(content) => {
            let todos = content.split("\n").collect::<Vec<&str>>();
            let index = id.parse::<usize>().unwrap();
            match todos.get(index) {
                Some(todo) => println!("{}", todo),
                None => println!("No todo found!"),
            }
        }
        Err(_) => println!("asd"),
    }
}

fn update(Id(id): Id, input: String) {
    match fs::read_to_string(get_file()) {
        Ok(content) => {
            let mut todos = content.split("\n").collect::<Vec<&str>>();
            let index = id.parse::<usize>().unwrap();
            match todos.get(index) {
                Some(_) => {
                    todos[index] = &input.as_str();

                    write_to_todos(todos.join("\n"), false)
                }
                None => println!("No todo found!"),
            }
        }
        Err(_) => println!("asd"),
    }
}

fn delete(Id(id): Id) {
    println!("{}", id)
}

fn help() {
    println!("HELP");
}

pub fn handle_command(command: Command) {
    match command {
        Command::List => list(),
        Command::Add(input) => add(input),
        Command::Get(id) => get(id),
        Command::Update(id, input) => update(id, input),
        Command::Delete(id) => delete(id),
        Command::Help => help(),
    }

    exit(0)
}
