//! @interface-stub
use crate::records::type_checker::TypeChecker;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker {
    pub fn fresh_type_type_level(&mut self, level: TypeLevel) -> TypeId {
        unsafe {
            let module = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            (*module)
                .internal_types
                .fresh_type_not_null_builtin_types_type_level(&*self.builtin_types, level)
        }
    }
}
