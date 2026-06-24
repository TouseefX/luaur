use crate::records::internal_type_finder::InternalTypeFinder;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFinder {
    pub fn visit_type_id_pending_expansion_type(
        &mut self,
        _ty: TypeId,
        _pet: &PendingExpansionType,
    ) -> bool {
        LUAU_ASSERT!(false);
        false
    }
}
