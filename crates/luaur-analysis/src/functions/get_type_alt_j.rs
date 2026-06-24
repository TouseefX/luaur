//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/Type.h:1089:get`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:1089-1098, hand-ported)

use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> const T* get(TypeId tv)` — call as `get::<TableType>(tv)`.
pub unsafe fn get<T: TypeVariantMember + 'static>(tv: TypeId) -> *const T {
    LUAU_ASSERT!(!tv.is_null());

    if core::any::TypeId::of::<T>() != core::any::TypeId::of::<BoundType>() {
        LUAU_ASSERT!(BoundType::get_if(&(*tv).ty).is_none());
    }

    match T::get_if(&(*tv).ty) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}

#[allow(unused_imports)]
pub use get as get_type_id;
