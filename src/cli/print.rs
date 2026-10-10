use crate::execution::{ExecutionLog, ValidationErrorContext};

pub fn print_validation_error(error: ValidationErrorContext) {
    eprint!(
        "error from {}:\n  {}",
        error.defined_in.display(),
        error.error
    );
}

pub fn print_execution_log(log: ExecutionLog) {
    match log {
        ExecutionLog::DirectoryCreated(path) => {
            println!("created {}", path.display());
        }
        ExecutionLog::SymlinkRemoved(path) => {
            println!("unlinked {}", path.display());
        }
        ExecutionLog::SymlinkCreated {
            source_path,
            destination_path,
        } => {
            println!(
                "linked {} -> {}",
                source_path.display(),
                destination_path.display()
            );
        }
        ExecutionLog::Error {
            error,
            defined_in,
            source_path,
            destination_path,
        } => {
            eprintln!(
                "error from {}:\n  could not link {} -> {}: {}",
                defined_in.display(),
                source_path.display(),
                destination_path.display(),
                error
            );
        }
    }
}
