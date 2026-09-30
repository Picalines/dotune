use std::path::PathBuf;

use clap::Parser;

use crate::{command::RepoPath, shared::SimplePath};

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long = "root", default_value = ".", value_parser = parse_repo_path)]
    pub repo_path: RepoPath,

    #[arg(short, long, default_value = ".dotune.lua", value_parser = parse_filename)]
    pub config_name: PathBuf,
}

fn parse_repo_path(path_arg: &str) -> Result<RepoPath, String> {
    SimplePath::canonicalize(path_arg)
        .map_err(|error| error.to_string())
        .and_then(|simple_path| RepoPath::new(&simple_path).map_err(|error| error.to_string()))
}

fn parse_filename(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if path.file_name() == Some(path.as_os_str()) {
        Ok(path)
    } else {
        Err("must be a nonempty filename, not a path".to_string())
    }
}
