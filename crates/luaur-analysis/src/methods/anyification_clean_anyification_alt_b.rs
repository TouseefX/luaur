use crate::records::anyification::Anyification;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Anyification {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
        self.any_type_pack
    }
}
