//! Source: `Analysis/include/Luau/TypeOrPack.h:15-19` (hand-ported)
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};

/// C++ `get<T>(const TypeOrPack&)` for T a DIRECT member (TypeId/TypePackId).
/// (Signature-pinned overload name; the TypeVariant / TypePackVariant
/// overloads are `get_type_or_pack` / `get_type_or_pack_mut_2`.)
pub fn get_type_or_pack_mut<T: TypeOrPackMember>(ty_or_tp: &TypeOrPack) -> *const T {
    match T::get_if(ty_or_tp) {
        Some(r) => r as *const T,
        None => core::ptr::null(),
    }
}
