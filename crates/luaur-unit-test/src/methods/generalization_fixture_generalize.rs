use crate::records::generalization_fixture::GeneralizationFixture;
use luaur_analysis::functions::generalize::generalize;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;
use std::sync::Arc;

impl GeneralizationFixture {
    pub fn generalize(&mut self, ty: TypeId) -> Option<TypeId> {
        let scope = Arc::as_ptr(&self.scope) as *mut Scope;
        generalize(
            &mut *self.arena,
            &mut *self.builtin_types,
            scope,
            &mut self.generalized_types,
            ty,
            None,
        )
    }
}
