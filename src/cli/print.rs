use crate::execution::ValidationErrorContext;

pub fn print_validation_error(error: ValidationErrorContext) {
    eprint!(
        "error from {}:\n  {}",
        error.defined_in.display(),
        error.error
    );
}
