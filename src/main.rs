mod cli_args;
mod lua;

use clap::Parser;

fn main() {
    let args = cli_args::CliArgs::parse();

    let _ = match lua::run_config_scripts(&args) {
        Ok(config) => config,
        Err(error) => {
            dbg!(error);
            return;
        }
    };
}
