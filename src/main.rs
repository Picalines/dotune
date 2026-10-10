mod cli;
mod execution;
mod lua;
mod model;

use std::process::ExitCode;

use clap::Parser;
use execution::{ExecutionLog, ValidationContext, execute_commands, validate_instructions};

use crate::cli::{print_execution_log, print_validation_error};

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

    let mut has_errors = false;
    execute_commands(commands, |log| {
        has_errors |= matches!(log, ExecutionLog::Error { .. });
        print_execution_log(log);
    });

    if has_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
