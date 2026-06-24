//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Symbol.cpp:21:to_string`
//! Source: `Analysis/src/Symbol.cpp:21-28` (hand-ported)
extern crate alloc;

use crate::records::symbol::Symbol;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `std::string toString(const Symbol& name)`.
pub unsafe fn to_string(name: &Symbol) -> String {
    if !name.local.is_null() {
        return core::ffi::CStr::from_ptr((*name.local).name.value)
            .to_string_lossy()
            .into_owned();
    }

    LUAU_ASSERT!(!name.global.value.is_null());
    core::ffi::CStr::from_ptr(name.global.value)
        .to_string_lossy()
        .into_owned()
}

#[allow(unused_imports)]
pub use to_string as to_string_symbol;
