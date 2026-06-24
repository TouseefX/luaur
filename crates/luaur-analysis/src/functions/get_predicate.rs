//! Source: `Analysis/include/Luau/Predicate.h:87-91` (hand-ported)
use crate::type_aliases::predicate::{Predicate, PredicateMember};

/// C++ `template<typename T> const T* get(const Predicate& predicate)`.
pub fn get_predicate<T: PredicateMember>(predicate: &Predicate) -> *const T {
    match T::get_if(predicate) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
