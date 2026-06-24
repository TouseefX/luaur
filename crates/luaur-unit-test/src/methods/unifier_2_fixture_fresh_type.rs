//! @interface-stub
use crate::records::unifier_2_fixture::Unifier2Fixture;
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::records::free_type::FreeType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Unifier2Fixture {
    pub fn fresh_type(&mut self) -> (TypeId, *mut FreeType) {
        let ty = self
            .arena
            .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
                &mut *self.scope,
                self.builtin_types.neverType,
                self.builtin_types.unknownType,
                Polarity::Unknown,
            ));
        let free = unsafe { get_mutable_type_id::<FreeType>(ty) };
        assert!(!free.is_null());
        (ty, free)
    }
}
