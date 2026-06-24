use crate::records::time_limit_error::TimeLimitError;
use crate::records::type_checker::TypeChecker;

impl TypeChecker {
    pub fn throw_time_limit_error(&mut self) {
        let module_name = unsafe {
            if self.ice_handler.is_null() {
                // Fallback: shouldn't happen, but keep behavior well-defined for wasm builds.
                // If InternalErrorReporter is ever null, ICE-style code would likely be invalid anyway.
                return;
            }
            let handler = &*self.ice_handler;
            (*handler).module_name.clone()
        };

        panic!(
            "{:?}",
            TimeLimitError::time_limit_error_time_limit_error(&module_name)
        );
    }
}
