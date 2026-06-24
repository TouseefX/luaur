//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:275-281` (hand-ported)
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> const T* get(TypeFunctionTypeId tv)`.
pub fn get_type_function_type_id<T: TypeFunctionTypeVariantMember>(
    tv: TypeFunctionTypeId,
) -> *const T {
    LUAU_ASSERT!(!tv.is_null());
    if tv.is_null() {
        return core::ptr::null();
    }
    unsafe {
        match T::get_if(&(*tv).type_variant) {
            Some(r) => r as *const T,
            None => core::ptr::null(),
        }
    }
}
