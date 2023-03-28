use std::process::exit;

use crate::validate::{ADD, DELETE, GET, UPDATE};

pub enum ArgsError {
    NoCommand,
    NonValidCommand(String),
    UpdateWithoutNewValue,
    NotEnoughArgument,
}

pub fn handle_errors(error: ArgsError) {
    match error {
        ArgsError::NoCommand => {
            print!(
                "No Command has been specified! Try: {}, {}, {}, {}.",
                ADD, GET, UPDATE, DELETE
            )
        }
        ArgsError::NonValidCommand(error_message) => print!(
            "Command do not exists: {}! Try: {}, {}, {}, {}.",
            error_message, ADD, GET, UPDATE, DELETE
        ),
        ArgsError::UpdateWithoutNewValue => print!("Please specify the new value for that todo!"),
        ArgsError::NotEnoughArgument => print!("Command need more parameter."),
    }

    exit(1)
}
