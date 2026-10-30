use core::fmt::Display;
use std::path::Path;

use super::Printer;
use crate::execution::{ExecutionLog, ValidationErrorContext};

pub struct PorcelainPrinter;

impl Printer for PorcelainPrinter {
    fn error(&self, error: impl Display) {
        eprintln!("error {error}");
    }

    fn validation_error(&mut self, error: ValidationErrorContext) {
        self.error(error.error);
    }

    fn execution_log(&mut self, _defined_in: &Path, log: ExecutionLog) {
        match log {
            ExecutionLog::DirectoryCreated(path) => {
                print!("dir-created\0{}\0\0", path.display());
            }
            ExecutionLog::PathDeleted(path) => {
                print!("path-deleted\0{}\0\0", path.display());
            }
            ExecutionLog::SymlinkCreated {
                source_path,
                destination_path,
            } => {
                print!(
                    "symlink-created\0{}\0{}\0\0",
                    source_path.display(),
                    destination_path.display()
                );
            }
            ExecutionLog::SymlinkRecreated {
                source_path,
                destination_path,
            } => {
                print!(
                    "symlink-recreated\0{}\0{}\0\0",
                    source_path.display(),
                    destination_path.display()
                );
            }
            ExecutionLog::Copied {
                source_path,
                destination_path,
            } => {
                print!(
                    "copied-over\0{}\0{}\0\0",
                    source_path.display(),
                    destination_path.display()
                );
            }
            ExecutionLog::Error {
                operation,
                error,
                source_path,
                destination_path,
            } => {
                print!(
                    "error\0{operation}\0{}\0{}\0\0",
                    source_path.display(),
                    destination_path.display()
                );
                self.error(error);
            }
        }
    }
}
