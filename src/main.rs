mod cli_args;
mod command;
mod lua;
mod shared;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let args = cli_args::CliArgs::parse();

    match lua::run_config_scripts(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
