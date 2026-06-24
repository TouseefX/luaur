use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::internal_error_reporter::InternalErrorReporter;
use luaur_ast::records::location::Location;

impl InternalErrorReporter {
    pub fn ice_string_location(&self, message: &str, location: &Location) {
        let error = InternalCompilerError::internal_compiler_error_string_string_location(
            alloc::string::String::from(message),
            self.module_name.clone(),
            *location,
        );

        if let Some(ref on_internal_error) = self.on_internal_error {
            let msg = unsafe { core::ffi::CStr::from_ptr(error.what()).to_string_lossy() };
            on_internal_error(&msg);
        }

        std::panic::panic_any(error);
    }
}
