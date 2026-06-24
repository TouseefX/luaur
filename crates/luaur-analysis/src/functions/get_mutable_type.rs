//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/Type.h:1101:get_mutable`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:1101-1110, hand-ported)

use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariantMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `template<typename T> T* getMutable(TypeId tv)` — call as `getMutable::<TableType>(tv)`.
#[allow(non_snake_case)]
pub unsafe fn getMutable<T: TypeVariantMember + 'static>(tv: TypeId) -> *mut T {
    LUAU_ASSERT!(!tv.is_null());

    if core::any::TypeId::of::<T>() != core::any::TypeId::of::<BoundType>() {
        LUAU_ASSERT!(BoundType::get_if(&(*tv).ty).is_none());
    }

    match T::get_if_mut(&mut (*as_mutable_type_id(tv)).ty) {
        Some(r) => r as *mut T,
        None => core::ptr::null_mut(),
    }
}

#[allow(unused_imports)]
pub use getMutable as get_mutable_type_id;
#[allow(unused_imports)]
pub use getMutable as get_mutable;
