use crate::records::instantiation::Instantiation;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Instantiation {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        LUAU_ASSERT!(false);
        tp
    }
}
