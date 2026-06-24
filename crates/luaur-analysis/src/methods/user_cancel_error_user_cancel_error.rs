use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::user_cancel_error::UserCancelError;
use alloc::string::String;

impl UserCancelError {
    pub fn new(module_name: String) -> Self {
        Self {
            base: InternalCompilerError {
                message: String::from("Analysis has been cancelled by user"),
                module_name: Some(module_name),
                location: None,
            },
        }
    }
}
