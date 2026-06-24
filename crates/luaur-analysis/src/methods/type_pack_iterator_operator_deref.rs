use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypePackIterator {
    pub fn operator_deref(&self) -> &TypeId {
        LUAU_ASSERT!(!self.tp.is_null());
        unsafe {
            let tp = &*self.tp;
            &tp.head[self.currentIndex]
        }
    }
}
