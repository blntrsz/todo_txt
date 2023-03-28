use crate::{
    command::{Command, Id},
    error::ArgsError,
};

pub const ADD: &str = "add";
pub const GET: &str = "get";
pub const UPDATE: &str = "update";
pub const DELETE: &str = "delete";
pub const HELP: &str = "help";
pub const DASH_HELP: &str = "-h";
pub const DASH_DASH_HELP: &str = "--help";

pub fn validate_arguments(args: Vec<String>) -> Result<Command, ArgsError> {
    if args.len() < 2 {
        return Err(ArgsError::NoCommand);
    }

    if args.len() < 3 {
        return match args[1].as_str() {
            HELP | DASH_DASH_HELP | DASH_HELP => Ok(Command::Help),
            ADD | GET | UPDATE | DELETE => Err(ArgsError::NotEnoughArgument),
            _ => Err(ArgsError::NonValidCommand(args[1].clone())),
        };
    }

    return match args[1].as_str() {
        ADD => Ok(Command::Add(args[2].clone())),
        GET => Ok(Command::Get(Id(args[2].clone()))),
        UPDATE => {
            if args.len() < 4 {
                return Err(ArgsError::UpdateWithoutNewValue);
            }

            return Ok(Command::Update(Id(args[2].clone()), args[3].clone()));
        }
        DELETE => Ok(Command::Delete(Id(args[2].clone()))),
        _ => Err(ArgsError::NonValidCommand(args[2].clone())),
    };
}
