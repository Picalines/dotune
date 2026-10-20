use clap::Parser;

use crate::model::{AbsolutePathBuf, RepoPath};

/// Manage dotfiles with Lua scripts
#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Dotfile repository containing .dotune.lua
    #[arg(default_value = ".", value_parser = parse_repo_path)]
    pub repo_path: RepoPath,

    /// Don't apply write operations to the file system
    #[arg(long, short = 'n')]
    pub dry_run: bool,

    /// Script input parameter (repeat for multiple parameters)
    #[arg(short = 'p', long = "param", value_name = "NAME=VALUE", value_parser = parse_param)]
    pub params: Vec<(String, String)>,
}

fn parse_repo_path(path_arg: &str) -> Result<RepoPath, String> {
    let path = AbsolutePathBuf::canonicalize(path_arg).map_err(|error| error.to_string())?;
    RepoPath::new(&path).map_err(|error| error.to_string())
}
fn parse_param(value: &str) -> Result<(String, String), String> {
    let (name, value) = value
        .split_once('=')
        .filter(|(name, _)| !name.is_empty())
        .ok_or_else(|| "expected NAME=VALUE with a non-empty name".to_string())?;

    Ok((name.to_owned(), value.to_owned()))
}
