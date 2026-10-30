use core::fmt;
use core::fmt::Display;
use std::path::{Path, PathBuf};

use colored::Colorize;

use super::Printer;
use crate::execution::{ExecutionLog, ValidationErrorContext};
use crate::model::RelativeDisplay;

pub struct HumanPrinter {
    previous_defined_in: Option<PathBuf>,
}

impl HumanPrinter {
    pub fn new() -> Self {
        Self {
            previous_defined_in: None,
        }
    }

    fn eprint_config(&mut self, defined_in: &Path) {
        if self.previous_defined_in.as_deref() == Some(defined_in) {
            return;
        }

        let path = defined_in.relative_display().to_string().dimmed();

        eprintln!("{} {path}", Label::Comment);

        self.previous_defined_in = Some(defined_in.to_owned());
    }
}

impl Printer for HumanPrinter {
    fn error(&self, error: impl Display) {
        eprintln!("{} {error}", Label::Error);
    }

    fn validation_error(&mut self, error: ValidationErrorContext) {
        self.eprint_config(&error.defined_in);
        self.error(error.error);
    }

    fn execution_log(&mut self, defined_in: &Path, log: ExecutionLog) {
        self.eprint_config(defined_in);

        match log {
            ExecutionLog::DirectoryCreated(path) => {
                println!("{} {}", Label::Created, path.relative_display());
            }
            ExecutionLog::PathDeleted(path) => {
                println!("{} {}", Label::Deleted, path.relative_display());
            }
            ExecutionLog::SymlinkCreated {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Linked,
                    source_path.relative_display(),
                    Label::Arrow,
                    destination_path.relative_display()
                );
            }
            ExecutionLog::SymlinkRecreated {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Relinked,
                    source_path.relative_display(),
                    Label::Arrow,
                    destination_path.relative_display()
                );
            }
            ExecutionLog::Copied {
                source_path,
                destination_path,
            } => {
                println!(
                    "{} {} {} {}",
                    Label::Copied,
                    source_path.relative_display(),
                    Label::Arrow,
                    destination_path.relative_display()
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
                    source_path.relative_display(),
                    Label::Arrow,
                    destination_path.relative_display(),
                    error
                );
            }
        }
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
        let (marker, label) = match self {
            Self::Linked => (Some("+".green()), "ln".blue()),
            Self::Relinked => (Some("~".blue()), "ln".blue()),
            Self::Copied => (Some("+".green()), "cp".yellow()),
            Self::Deleted => (Some("-".red()), "rm".red()),
            Self::Created => (Some("+".green()), "mkdir".purple()),
            Self::Error => (None, "error".red()),
            Self::Arrow => (None, "->".blue()),
            Self::Comment => (None, "#".dimmed()),
        };

        if let Some(marker) = marker {
            write!(formatter, "{marker} ")?;
        }

        write!(formatter, "{label}")
    }
}
