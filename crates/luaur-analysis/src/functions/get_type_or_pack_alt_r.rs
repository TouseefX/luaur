//! Source: `Analysis/include/Luau/TypeOrPack.h:21-28` (hand-ported)
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_variant::TypeVariantMember;

/// C++ `get<T>(const TypeOrPack&)` for T a member of TypeVariant: unwrap the
/// TypeId member, then `get<T>(*ty)`.
pub fn get_type_or_pack<T: TypeVariantMember + 'static>(ty_or_tp: &TypeOrPack) -> *const T {
    match TypeId::get_if(ty_or_tp) {
        Some(ty) => unsafe { crate::functions::get_type_alt_j::get::<T>(*ty) },
        None => core::ptr::null(),
    }
}
