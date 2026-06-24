//! Source: `Analysis/include/Luau/TypeFunctionError.h:88-92` (hand-ported)
use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorDataMember;

/// C++ `template<typename T> T* get(TypeFunctionError& e)`.
pub fn get_type_function_error_mut<T: TypeFunctionErrorDataMember>(
    e: &mut TypeFunctionError,
) -> *mut T {
    match T::get_if_mut(&mut e.data) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}
