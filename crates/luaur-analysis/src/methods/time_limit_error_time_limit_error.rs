use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::time_limit_error::TimeLimitError;
use alloc::format;

impl TimeLimitError {
    pub fn time_limit_error_time_limit_error(module_name: &str) -> Self {
        Self {
            base: InternalCompilerError {
                message: format!("Typeinfer failed to complete in allotted time"),
                module_name: Some(module_name.to_owned()),
                location: None,
            },
        }
    }
}
