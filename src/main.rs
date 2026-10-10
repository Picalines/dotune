mod cli;
mod execution;
mod lua;
mod model;

use clap::Parser;
use execution::{ValidationContext, execute_commands, validate_instructions};
use std::process::ExitCode;

use crate::cli::print_validation_error;

fn main() -> ExitCode {
    let args = cli::Args::parse();

    let instructions = match lua::run_config_scripts(&args) {
        Ok(instructions) => instructions,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let commands = match validate_instructions(
        instructions,
        ValidationContext {
            repo_path: &args.repo_path,
        },
    ) {
        Ok(commands) => commands,
        Err(error_context) => {
            print_validation_error(error_context);
            return ExitCode::FAILURE;
        }
    };

    execute_commands(commands);

    ExitCode::SUCCESS
}
