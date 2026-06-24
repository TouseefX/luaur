//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypePack.h:215:get_mutable`
//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:215-224, hand-ported)

use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> T* getMutable(TypePackId tp)`.
#[allow(non_snake_case)]
pub unsafe fn getMutable<T: TypePackVariantMember + 'static>(tp: TypePackId) -> *mut T {
    LUAU_ASSERT!(!tp.is_null());

    if core::any::TypeId::of::<T>() != core::any::TypeId::of::<BoundTypePack>() {
        LUAU_ASSERT!(BoundTypePack::get_if(&(*tp).ty).is_none());
    }

    match T::get_if_mut(&mut (*as_mutable_type_pack_id(tp)).ty) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use getMutable as get_mutable_type_pack_id;
