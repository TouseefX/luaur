//! Source: `Analysis/include/Luau/Error.h:718-722` (hand-ported)
use crate::records::type_error::TypeError;
use crate::type_aliases::type_error_data::TypeErrorDataMember;

/// C++ `template<typename T> T* get(TypeError& e)`.
pub fn get_type_error_mut<T: TypeErrorDataMember>(e: &mut TypeError) -> *mut T {
    match T::get_if_mut(&mut e.data) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}
