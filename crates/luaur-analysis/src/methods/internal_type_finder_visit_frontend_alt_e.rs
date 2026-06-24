use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::internal_type_finder::InternalTypeFinder;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFinder {
    pub fn visit_type_pack_id_blocked_type_pack(
        &mut self,
        _tp: TypePackId,
        _btp: &BlockedTypePack,
    ) -> bool {
        LUAU_ASSERT!(false);
        false
    }
}
