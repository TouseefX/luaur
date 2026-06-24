use crate::records::weird_iter::WeirdIter;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl WeirdIter {
    pub fn weird_iter_operator_deref(&mut self) -> &mut TypeId {
        LUAU_ASSERT!(self.weird_iter_good());
        unsafe {
            let pack = &mut *self.pack;
            &mut pack.head[self.index]
        }
    }
}
