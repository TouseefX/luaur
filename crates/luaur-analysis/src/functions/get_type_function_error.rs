//! Source: `Analysis/include/Luau/TypeFunctionError.h:82-86` (hand-ported)
use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorDataMember;

/// C++ `template<typename T> const T* get(const TypeFunctionError& e)`.
pub fn get_type_function_error<T: TypeFunctionErrorDataMember>(e: &TypeFunctionError) -> *const T {
    match T::get_if(&e.data) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
