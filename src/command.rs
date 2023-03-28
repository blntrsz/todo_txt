use std::process::exit;

pub struct Id(pub String);

pub enum Command {
    Add(String),
    Get(Id),
    Update(Id, String),
    Delete(Id),
    Help,
}

fn add(input: String) {
    println!("{}", input);
}
fn get(Id(id): Id) {
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
        Command::Add(input) => add(input),
        Command::Get(id) => get(id),
        Command::Update(id, input) => update(id, input),
        Command::Delete(id) => delete(id),
        Command::Help => help(),
    }

    exit(0)
}
