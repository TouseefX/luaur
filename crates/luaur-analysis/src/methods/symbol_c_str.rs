use crate::records::symbol::Symbol;

impl Symbol {
    #[inline]
    pub fn c_str(&self) -> *const core::ffi::c_char {
        if !self.local.is_null() {
            unsafe { (*self.local).name.value }
        } else {
            luaur_common::macros::luau_assert::LUAU_ASSERT!(!self.global.value.is_null());
            self.global.value
        }
    }
}
