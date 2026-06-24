//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:239:get_mutable`
//! Source: `Analysis/include/Luau/TypeUtils.h:238-245` (hand-ported)

use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;
use crate::type_aliases::type_variant::TypeVariantMember;

/// Dispatch of the inner `getMutable<T>(*ty)` per id type `Ty`: C++ resolves
/// the overload by argument type; Rust needs the trait.
pub trait GetMutableThroughId<Ty: Copy>: Sized {
    /// # Safety
    /// `ty` must be a valid id pointer (the C++ overloads dereference it).
    unsafe fn get_mutable_through(ty: Ty) -> *mut Self;
}

impl<T: TypeVariantMember + 'static> GetMutableThroughId<TypeId> for T {
    unsafe fn get_mutable_through(ty: TypeId) -> *mut T {
        get_mutable_type_id::<T>(ty)
    }
}

impl<T: TypePackVariantMember + 'static> GetMutableThroughId<TypePackId> for T {
    unsafe fn get_mutable_through(tp: TypePackId) -> *mut T {
        get_mutable_type_pack_id::<T>(tp)
    }
}

/// C++ `template<typename T, typename Ty> T* getMutable(std::optional<Ty> ty)`.
pub unsafe fn get_mutable_optional_ty<T: GetMutableThroughId<Ty>, Ty: Copy>(
    ty: Option<Ty>,
) -> *mut T {
    if let Some(ty) = ty {
        T::get_mutable_through(ty)
    } else {
        core::ptr::null_mut()
    }
}
