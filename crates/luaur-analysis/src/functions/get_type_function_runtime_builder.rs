//! Source: `Analysis/include/Luau/TypeFunctionRuntimeBuilder.h:14-18` (hand-ported)
use crate::type_aliases::type_function_kind::{TypeFunctionKind, TypeFunctionKindMember};

/// C++ `template<typename T> const T* get(const TypeFunctionKind& tfkind)`.
pub fn get_type_function_kind<T: TypeFunctionKindMember>(tfkind: &TypeFunctionKind) -> *const T {
    match T::get_if(tfkind) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
