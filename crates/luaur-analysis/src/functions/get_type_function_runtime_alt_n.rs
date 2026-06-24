//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> const T* get(TypeFunctionTypePackId tv)`.
pub fn get_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
    tv: TypeFunctionTypePackId,
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
