use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypePackIterator {
    pub fn tail(&self) -> Option<TypePackId> {
        LUAU_ASSERT!(self.tp.is_null());
        if !self.currentTypePack.is_null() {
            Some(self.currentTypePack)
        } else {
            None
        }
    }
}
