use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::internal_error_reporter::InternalErrorReporter;

impl InternalErrorReporter {
    pub fn ice_string(&self, message: &str) {
        let error = InternalCompilerError::internal_compiler_error_string_string(
            alloc::string::String::from(message),
            self.module_name.clone(),
        );

        if let Some(ref on_internal_error) = self.on_internal_error {
            let what = error.what();
            let msg = unsafe { core::ffi::CStr::from_ptr(what).to_string_lossy() };
            on_internal_error(&msg);
        }

        std::panic::panic_any(error);
    }
}
