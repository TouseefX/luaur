//! @interface-stub
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use alloc::sync::Arc;
use luaur_ast::enums::mode::Mode;

impl TypeChecker {
    pub fn report_error_type_error(&mut self, error: &TypeError) {
        let module = self.current_module.as_ref().unwrap();

        if module.mode == Mode::NoCheck {
            return;
        }

        unsafe {
            let module = Arc::as_ptr(module) as *mut crate::records::module::Module;
            (*module).errors.push(error.clone());
            (*module).errors.last_mut().unwrap().module_name = (*module).name.clone();
        }
    }
}
