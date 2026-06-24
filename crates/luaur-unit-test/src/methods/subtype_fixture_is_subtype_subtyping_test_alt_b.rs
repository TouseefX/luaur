use crate::records::subtype_fixture::SubtypeFixture;
use alloc::sync::Arc;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::subtyping_result::SubtypingResult;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl SubtypeFixture {
    pub fn is_subtype_type_pack_id_type_pack_id(
        &mut self,
        sub_ty: TypePackId,
        super_ty: TypePackId,
    ) -> SubtypingResult {
        let scope = Arc::as_ptr(&self.root_scope) as *mut Scope;
        let bindable_generics: Vec<TypeId> = Vec::new();
        self.subtyping
            .is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
                sub_ty,
                super_ty,
                scope,
                &bindable_generics,
            )
    }
}
