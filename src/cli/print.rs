use core::fmt;
use core::fmt::Display;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use std::sync::Arc;

use colored::Colorize;

use crate::execution::{ExecutionLog, ValidationErrorContext};
use crate::model::SystemInfo;

pub struct Printer {
    system_info: Arc<SystemInfo>,
    previous_defined_in: Option<PathBuf>,
}

impl Printer {
    pub fn new(system_info: Arc<SystemInfo>) -> Self {
        Self {
            system_info,
            previous_defined_in: None,
        }
    }

    pub fn print_error(&self, error: impl Display) {
        eprintln!("{} {error}", Label::Error);
    }

    pub fn print_validation_error(&mut self, error: ValidationErrorContext) {
        self.eprint_config(&error.defined_in);
        self.print_error(error.error);
    }

    pub fn print_execution_log(&mut self, defined_in: &Path, log: ExecutionLog) {
        self.eprint_config(defined_in);

        match log {
            ExecutionLog::DirectoryCreated(path) => {
                println!("{} {}", Label::Created, self.display_path(&path));
            }
            ExecutionLog::PathDeleted(path) => {
                println!("{} {}", Label::Deleted, self.display_path(&path));
            }
            ExecutionLog::SymlinkCreated {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Linked,
                    self.display_path(&source_path),
                    Label::Arrow,
                    self.display_path(&destination_path)
                );
            }
            ExecutionLog::SymlinkRecreated {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Relinked,
                    self.display_path(&source_path),
                    Label::Arrow,
                    self.display_path(&destination_path)
                );
            }
            ExecutionLog::Copied {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Copied,
                    self.display_path(&source_path),
                    Label::Arrow,
                    self.display_path(&destination_path)
                );
            }
            ExecutionLog::Error {
                operation,
                error,
                source_path,
                destination_path,
            } => {
                eprintln!(
                    "{} could not {} {} {} {}: {}",
                    Label::Error,
                    operation,
                    self.display_path(&source_path),
                    Label::Arrow,
                    self.display_path(&destination_path),
                    error
                );
            }
        }
    }

    fn eprint_config(&mut self, defined_in: &Path) {
        if self.previous_defined_in.as_deref() == Some(defined_in) {
            return;
        }

        let path = self.display_path(defined_in).to_string().dimmed();

        eprintln!("{} {path}", Label::Comment);

        self.previous_defined_in = Some(defined_in.to_owned());
    }

    fn display_path(&self, path: &Path) -> impl fmt::Display {
        fmt::from_fn(|formatter| {
            if let Ok(relative) = path.strip_prefix(self.system_info.cwd.as_ref()) {
                return relative.display().fmt(formatter);
            }

            if let Ok(relative) = path.strip_prefix(self.system_info.home.as_ref()) {
                formatter.write_str("~")?;
                if relative.is_empty() {
                    return Ok(());
                }
                return write!(formatter, "{}{}", MAIN_SEPARATOR, relative.display());
            }

            path.display().fmt(formatter)
        })
    }
}

enum Label {
    Linked,
    Relinked,
    Copied, // TODO: would be correct to tell "copied over" for dirs
    Deleted,
    Created,
    Error,
    Arrow,
    Comment,
}

impl fmt::Display for Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Linked => "ln".purple(),
            Self::Relinked => "ln".blue(),
            Self::Copied => "cp".yellow(),
            Self::Deleted => "rm".yellow(),
            Self::Created => "mkdir".purple(),
            Self::Error => "error".red(),
            Self::Arrow => "->".blue(),
            Self::Comment => "#".dimmed(),
        };

        fmt::Display::fmt(&label, formatter)
    }
}
