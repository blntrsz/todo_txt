mod command;
mod error;
mod validate;

use command::handle_command;
use error::handle_errors;
use std::env;
use validate::validate_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_arguments(args) {
        Ok(command) => handle_command(command),
        Err(error) => handle_errors(error),
    };
}
