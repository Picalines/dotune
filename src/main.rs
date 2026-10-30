mod cli;
mod execution;
mod lua;
mod model;

use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use execution::{ExecutionLog, ValidationContext, execute_commands, validate_instructions};

use crate::cli::{Args, HumanPrinter, OutputFormat, Printer};
use crate::model::{Inputs, SystemInfo};

fn main() -> ExitCode {
    let args = Args::parse();

    match args.output {
        OutputFormat::Human => run(args, HumanPrinter::new()),
    }
}

fn run(args: Args, mut printer: impl Printer) -> ExitCode {
    let system_info = Arc::new(SystemInfo::detect());

    let inputs = match Inputs::new(args.params) {
        Ok(inputs) => Arc::new(inputs),
        Err(error) => {
            printer.error(error);
            return ExitCode::FAILURE;
        }
    };

    let instructions = match lua::run_config_scripts(&args.repo_path, system_info, inputs) {
        Ok(instructions) => instructions,
        Err(error) => {
            // TODO: nice file path to a script?
            printer.error(error);
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
            printer.validation_error(error_context);
            return ExitCode::FAILURE;
        }
    };

    let mut has_errors = false;
    execute_commands(commands, args.dry_run, |defined_in, log| {
        has_errors |= matches!(log, ExecutionLog::Error { .. });
        printer.execution_log(defined_in, log);
    });

    if has_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
