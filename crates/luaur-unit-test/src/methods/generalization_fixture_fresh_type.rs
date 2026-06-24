use crate::records::generalization_fixture::GeneralizationFixture;
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::records::free_type::FreeType;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;
use std::sync::Arc;

impl GeneralizationFixture {
    pub fn fresh_type(&mut self) -> (TypeId, *mut FreeType) {
        let scope = Arc::as_ptr(&self.scope) as *mut Scope;
        let ty = self
            .arena
            .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
                scope,
                self.builtin_types.neverType,
                self.builtin_types.unknownType,
                Polarity::Unknown,
            ));
        let free = unsafe { get_mutable_type_id::<FreeType>(ty) };
        assert!(!free.is_null());
        (ty, free)
    }
}
