use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long = "root", default_value = ".", value_parser = parse_existing_dir)]
    pub root_dir: PathBuf,

    #[arg(short, long, default_value = ".dotune.lua", value_parser = parse_filename)]
    pub config_name: PathBuf,
}

fn parse_existing_dir(path_arg: &str) -> Result<PathBuf, String> {
    let Ok(path) = std::fs::canonicalize(path_arg) else {
        return Err(format!("'{}' is not an existing path", path_arg));
    };

    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path_arg));
    }

    Ok(path)
}

fn parse_filename(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if path.file_name() == Some(path.as_os_str()) {
        Ok(path)
    } else {
        Err("must be a nonempty filename, not a path".to_string())
    }
}
