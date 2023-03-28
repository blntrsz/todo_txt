use std::{env, process::exit};

struct Id(String);

enum Command {
    Add(String),
    Get(Id),
    Update(Id, String),
    Delete(Id),
    Help,
}

enum ArgsError {
    NoCommand,
    NonValidCommand(String),
    UpdateWithoutNewValue,
    NotEnoughArgument,
}

fn handle_errors(error: ArgsError) {
    match error {
        ArgsError::NoCommand => {
            print!("No Command has been specified! Try: add, get, update, delete.")
        }
        ArgsError::NonValidCommand(error_message) => print!(
            "Command do not exists: {}! Try: add, get, update, delete.",
            error_message
        ),
        ArgsError::UpdateWithoutNewValue => print!("Please specify the new value for that todo!"),
        ArgsError::NotEnoughArgument => print!("Command need more parameter."),
    }

    exit(1)
}

fn validate_arguments(args: Vec<String>) -> Result<Command, ArgsError> {
    if args.len() < 2 {
        return Err(ArgsError::NoCommand);
    }

    if args.len() < 3 {
        return match args[1].as_str() {
            "help" => Ok(Command::Help),
            "--help" => Ok(Command::Help),
            "-h" => Ok(Command::Help),
            "add" => Err(ArgsError::NotEnoughArgument),
            "get" => Err(ArgsError::NotEnoughArgument),
            "update" => Err(ArgsError::NotEnoughArgument),
            "delete" => Err(ArgsError::NotEnoughArgument),
            _ => Err(ArgsError::NonValidCommand(args[1].clone())),
        };
    }

    return match args[1].as_str() {
        "add" => Ok(Command::Add(args[2].clone())),
        "get" => Ok(Command::Get(Id(args[2].clone()))),
        "update" => {
            if args.len() < 4 {
                return Err(ArgsError::UpdateWithoutNewValue);
            }

            return Ok(Command::Update(Id(args[2].clone()), args[3].clone()));
        }
        "delete" => Ok(Command::Delete(Id(args[2].clone()))),
        _ => Err(ArgsError::NonValidCommand(args[2].clone())),
    };
}

fn handle_command(command: Command) {
    match command {
        Command::Add(input) => println!("{}", input),
        Command::Get(Id(id)) => println!("{}", id),
        Command::Update(Id(id), input) => println!("{}, {}", id, input),
        Command::Delete(Id(id)) => println!("{}", id),
        Command::Help => println!("{}", "help"),
    }

    exit(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_arguments(args) {
        Ok(command) => handle_command(command),
        Err(error) => handle_errors(error),
    };
}
