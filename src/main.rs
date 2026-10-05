mod cli_args;
mod execution;
mod lua;
mod model;

use clap::Parser;
use execution::{ValidationContext, validate};
use model::Command;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = cli_args::CliArgs::parse();

    let instructions = match lua::run_config_scripts(&args) {
        Ok(instructions) => instructions,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let mut exit_code = ExitCode::SUCCESS;

    let mut commands: Vec<Command> = Vec::with_capacity(instructions.len());

    let validation_context = ValidationContext {
        repo_path: &args.repo_path,
    };

    for instruction in instructions {
        match validate(&validation_context, instruction) {
            Ok(command) => commands.push(command),
            Err(error) => {
                exit_code = ExitCode::FAILURE;
                eprintln!(
                    "error from {}:\n  {}",
                    error.defined_in.display(),
                    error.error
                );
            }
        }
    }

    dbg!(commands);

    exit_code
}
