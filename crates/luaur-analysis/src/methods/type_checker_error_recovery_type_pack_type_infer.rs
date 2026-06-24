use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    pub fn error_recovery_type_pack_scope_ptr(&mut self, _scope: ScopePtr) -> TypePackId {
        unsafe { (*self.builtin_types).errorTypePack }
    }
}
