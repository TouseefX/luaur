//! Source: `Analysis/include/Luau/LValue.h:40-44` (hand-ported)
use crate::type_aliases::l_value::{LValue, LValueMember};

/// C++ `template<typename T> const T* get(const LValue& lvalue)`.
pub fn get_l_value<T: LValueMember>(lvalue: &LValue) -> *const T {
    match T::get_if(lvalue) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
