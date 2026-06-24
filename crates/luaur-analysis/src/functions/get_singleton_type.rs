//! Source: `Analysis/include/Luau/Type.h:252-259` (hand-ported)
use crate::records::singleton_type::SingletonType;
use crate::type_aliases::singleton_variant::SingletonVariantMember;

/// C++ `template<typename T> const T* get(const SingletonType* stv)`.
pub fn get_singleton_type<T: SingletonVariantMember>(stv: *const SingletonType) -> *const T {
    if stv.is_null() {
        return core::ptr::null();
    }
    unsafe {
        match T::get_if(&(*stv).variant) {
            Some(r) => r as *const T,
            None => core::ptr::null(),
        }
    }
}
