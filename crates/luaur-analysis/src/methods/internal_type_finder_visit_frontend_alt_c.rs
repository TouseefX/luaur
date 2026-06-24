use crate::records::free_type::FreeType;
use crate::records::internal_type_finder::InternalTypeFinder;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFinder {
    pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
        LUAU_ASSERT!(false);
        false
    }
}
