use clap::Parser;

use crate::{command::RepoPath, shared::SimplePath};

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long = "root", default_value = ".", value_parser = parse_repo_path)]
    pub repo_path: RepoPath,
}

fn parse_repo_path(path_arg: &str) -> Result<RepoPath, String> {
    SimplePath::canonicalize(path_arg)
        .map_err(|error| error.to_string())
        .and_then(|simple_path| RepoPath::new(&simple_path).map_err(|error| error.to_string()))
}
