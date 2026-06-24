use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    pub fn fresh_type_pack_scope_ptr(&mut self, scope: ScopePtr) -> TypePackId {
        self.fresh_type_pack_type_level((*scope).level)
    }
}
