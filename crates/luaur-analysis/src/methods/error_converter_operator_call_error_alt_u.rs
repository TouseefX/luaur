use crate::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_18(&self, _e: &ConstraintSolvingIncompleteError) -> String {
        String::from(
            "Type inference failed to complete, you may see some confusing types and type errors.",
        )
    }
}
