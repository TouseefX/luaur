use crate::records::blocked_type::BlockedType;
use crate::records::internal_type_finder::InternalTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl InternalTypeFinder {
    pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
        luaur_common::macros::luau_assert::LUAU_ASSERT!(false);
        false
    }
}
