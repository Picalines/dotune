mod cli_args;
mod execution;
mod lua;
mod model;

use clap::Parser;
use execution::{ValidationContext, validate_instructions};
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

    let commands = match validate_instructions(
        instructions,
        ValidationContext {
            repo_path: &args.repo_path,
        },
    ) {
        Ok(commands) => commands,
        Err(error_context) => {
            eprintln!(
                "error from {}:\n  {}",
                error_context.defined_in.display(),
                error_context.error
            );
            return ExitCode::FAILURE;
        }
    };

    dbg!(commands);

    ExitCode::SUCCESS
}
