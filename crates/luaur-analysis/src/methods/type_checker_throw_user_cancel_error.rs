use crate::records::type_checker::TypeChecker;
use crate::records::user_cancel_error::UserCancelError;

impl TypeChecker {
    pub fn throw_user_cancel_error(&mut self) {
        let module_name = unsafe {
            if self.ice_handler.is_null() {
                // Fallback: shouldn't happen, but keep behavior well-defined for wasm builds.
                // If InternalErrorReporter is ever null, ICE-style code would likely be invalid anyway.
                return;
            }
            let handler = &*self.ice_handler;
            (*handler).module_name.clone()
        };

        panic!("{:?}", UserCancelError::new(module_name));
    }
}
