//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypePack.h:204:get`
//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:204-213, hand-ported)

use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> const T* get(TypePackId tp)`.
pub unsafe fn get<T: TypePackVariantMember + 'static>(tp: TypePackId) -> *const T {
    LUAU_ASSERT!(!tp.is_null());

    if core::any::TypeId::of::<T>() != core::any::TypeId::of::<BoundTypePack>() {
        LUAU_ASSERT!(BoundTypePack::get_if(&(*tp).ty).is_none());
    }

    match T::get_if(&(*tp).ty) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}

#[allow(unused_imports)]
pub use get as get_type_pack_id;
