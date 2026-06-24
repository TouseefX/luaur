//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:283:get_mutable`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:283-289` (hand-ported)

use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> T* getMutable(TypeFunctionTypeId tv)`.
pub unsafe fn get_mutable_type_function_type_id<T: TypeFunctionTypeVariantMember>(
    tv: TypeFunctionTypeId,
) -> *mut T {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
        return core::ptr::null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionType*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionType)).type_variant) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use get_mutable_type_function_type_id as get_mutable;
