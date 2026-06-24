//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:88:get_mutable`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:88-94` (hand-ported)

use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::type_aliases::type_function_singleton_variant::TypeFunctionSingletonVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> T* getMutable(const TypeFunctionSingletonType* tv)`.
pub unsafe fn get_mutable_type_function_singleton_type<T: TypeFunctionSingletonVariantMember>(
    tv: *const TypeFunctionSingletonType,
) -> *mut T {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
        return core::ptr::null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionSingletonType*>(tv)->variant)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionSingletonType)).variant) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use get_mutable_type_function_singleton_type as get_mutable;
