mod cli_args;
mod lua;
mod model;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let args = cli_args::CliArgs::parse();

    let commands = match lua::run_config_scripts(&args) {
        Ok(commands) => commands,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    dbg!(commands);

    ExitCode::SUCCESS
}
