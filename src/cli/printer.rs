use core::fmt::Display;
use std::path::Path;

use crate::execution::{ExecutionLog, ValidationErrorContext};

pub trait Printer {
    fn error(&self, error: impl Display);

    fn validation_error(&mut self, error: ValidationErrorContext);

    fn execution_log(&mut self, defined_in: &Path, log: ExecutionLog);
}
