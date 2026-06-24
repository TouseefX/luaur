use crate::records::weird_iter::WeirdIter;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl WeirdIter {
    pub fn weird_iter_tail(&self) -> Option<TypePackId> {
        if self.pack.is_null() {
            return Some(self.pack_id);
        }
        LUAU_ASSERT!(self.index == unsafe { (*self.pack).head.len() });
        unsafe { (*self.pack).tail }
    }
}
