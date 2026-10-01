use clap::Parser;

use crate::model::{AbsolutePathBuf, RepoPath};

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long = "root", default_value = ".", value_parser = parse_repo_path)]
    pub repo_path: RepoPath,
}

fn parse_repo_path(path_arg: &str) -> Result<RepoPath, String> {
    let path = AbsolutePathBuf::canonicalize(path_arg).map_err(|error| error.to_string())?;
    RepoPath::new(&path).map_err(|error| error.to_string())
}
