//! Source: `Analysis/src/TypeInfer.cpp:5585-5588` (hand-ported)
//! C++ `TypeId TypeChecker::addTV(Type&& tv) { return currentModule->internalTypes.addType(std::move(tv)); }`.
use crate::records::module::Module;
use crate::records::r#type::Type;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;

impl TypeChecker {
    pub fn add_tv(&mut self, tv: Type) -> TypeId {
        // currentModule->internalTypes.addType(std::move(tv))
        unsafe {
            let module =
                Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
            (*module).internal_types.add_tv(tv)
        }
    }
}
