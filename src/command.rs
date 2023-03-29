use std::{
    fs,
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

fn add(input: String) {
    println!("{}", input);
}

fn get(Id(id): Id) {
    match fs::read_to_string(get_file()) {
        Ok(content) => {
            let todos = content
                .split("\n")
                .map(|c| c.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            let todo = todos.iter().find(|c| match c.get(0) {
                Some(value) => value.to_string() == id,
                None => false,
            });
            match todo {
                Some(value) => println!("{}", value.join(" ")),
                None => println!("Not found!"),
            }
        }
        Err(_) => println!("File does not exist!"),
    }
    println!("{}", id);
}

fn update(Id(id): Id, input: String) {
    println!("{}, {}", id, input)
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
