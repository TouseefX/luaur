//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:230:get`
//! Source: `Analysis/include/Luau/TypeUtils.h:229-236` (hand-ported)

use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;
use crate::type_aliases::type_variant::TypeVariantMember;

/// Dispatch of the inner `get<T>(*ty)` per id type `Ty`: C++ resolves the
/// overload by argument type; Rust needs the trait.
pub trait GetThroughId<Ty: Copy>: Sized {
    /// # Safety
    /// `ty` must be a valid id pointer (the C++ overloads dereference it).
    unsafe fn get_through(ty: Ty) -> *const Self;
}

impl<T: TypeVariantMember + 'static> GetThroughId<TypeId> for T {
    unsafe fn get_through(ty: TypeId) -> *const T {
        get_type_id::<T>(ty)
    }
}

impl<T: TypePackVariantMember + 'static> GetThroughId<TypePackId> for T {
    unsafe fn get_through(tp: TypePackId) -> *const T {
        get_type_pack_id::<T>(tp)
    }
}

/// C++ `template<typename T, typename Ty> const T* get(std::optional<Ty> ty)`.
pub unsafe fn get_optional_ty<T: GetThroughId<Ty>, Ty: Copy>(ty: Option<Ty>) -> *const T {
    if let Some(ty) = ty {
        T::get_through(ty)
    } else {
        core::ptr::null()
    }
}
