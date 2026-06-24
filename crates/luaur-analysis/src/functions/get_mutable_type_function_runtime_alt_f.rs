//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:175:get_mutable`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:175-181` (hand-ported)

use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> T* getMutable(TypeFunctionTypePackId tv)`.
pub unsafe fn get_mutable_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
    tv: TypeFunctionTypePackId,
) -> *mut T {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
        return core::ptr::null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionTypePackVar*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionTypePackVar)).type_variant) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use get_mutable_type_function_type_pack_id as get_mutable;
