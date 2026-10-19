mod cli;
mod execution;
mod lua;
mod model;

use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use execution::{ExecutionLog, ValidationContext, execute_commands, validate_instructions};

use crate::cli::Printer;
use crate::model::SystemInfo;

fn main() -> ExitCode {
    let args = cli::Args::parse();

    let system_info = Arc::new(SystemInfo::detect());

    let mut printer = Printer::new();

    let instructions = match lua::run_config_scripts(&args, system_info) {
        Ok(instructions) => instructions,
        Err(error) => {
            // TODO: nice file path to a script?
            printer.print_error(error);
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
            printer.print_validation_error(error_context);
            return ExitCode::FAILURE;
        }
    };

    let mut has_errors = false;
    execute_commands(commands, args.dry_run, |defined_in, log| {
        has_errors |= matches!(log, ExecutionLog::Error { .. });
        printer.print_execution_log(defined_in, log);
    });

    if has_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
