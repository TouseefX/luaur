use crate::records::subtype_fixture::SubtypeFixture;
use alloc::sync::Arc;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::subtyping_result::SubtypingResult;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn is_subtype_type_id_type_id(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
    ) -> SubtypingResult {
        let scope = Arc::as_ptr(&self.root_scope) as *mut Scope;
        self.subtyping
            .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
    }
}
