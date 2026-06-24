use crate::records::internal_compiler_error::InternalCompilerError;

impl InternalCompilerError {
    pub fn internal_compiler_error_string_string(
        message: alloc::string::String,
        module_name: alloc::string::String,
    ) -> Self {
        Self {
            message,
            module_name: Some(module_name),
            location: None,
        }
    }
}
