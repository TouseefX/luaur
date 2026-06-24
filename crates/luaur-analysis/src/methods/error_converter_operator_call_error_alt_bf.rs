use crate::records::error_converter::ErrorConverter;
use crate::records::recursive_restraint_violation::RecursiveRestraintViolation;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_51(&self, _e: &RecursiveRestraintViolation) -> String {
        String::from("Recursive type being used with different parameters.")
    }
}
