use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker {
    pub fn error_recovery_type_scope_ptr(&mut self, _scope: &ScopePtr) -> TypeId {
        unsafe { (*self.builtin_types).errorType }
    }
}
