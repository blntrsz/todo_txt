use std::{env, process::exit};

enum ArgsError {
    NoArg(String),
    NotValid(String),
}

fn handle_errors(error: ArgsError) {
    match error {
        ArgsError::NotValid(error_message) => print!("Argument Error: {}", error_message),
        ArgsError::NoArg(error_message) => print!("Argument Error: {}", error_message),
    }

    exit(1);
}

struct Command {
    command: String,
    input: String,
}

fn validate_arguments(args: Vec<String>) -> Result<Command, ArgsError> {
    if args.len() < 2 {
        return Err(ArgsError::NoArg("No argument was given!".to_string()));
    }

    if args.len() < 3 {
        return Err(ArgsError::NotValid("Argument is not valid!".to_string()));
    }

    let command = args[1].clone();
    let input = args[2].clone();

    return Ok(Command { command, input });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_arguments(args) {
        Ok(command) => println!("{}, {}", command.command, command.input),
        Err(error) => handle_errors(error)
    };
}
