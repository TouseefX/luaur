use crate::records::type_checker::TypeChecker;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    /// C++ `TypePackId TypeChecker::addTypePack(TypePackVar&& tv)` (TypeInfer.cpp:5590):
    /// `return currentModule->internalTypes.addTypePack(std::move(tv));`
    pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
        unsafe {
            (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types
                .add_type_pack_type_pack_var(tp)
        }
    }
}
