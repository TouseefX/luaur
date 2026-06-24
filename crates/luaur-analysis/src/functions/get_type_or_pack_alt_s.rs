//! Source: `Analysis/include/Luau/TypeOrPack.h:30-37` (hand-ported)
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;

/// C++ `get<T>(const TypeOrPack&)` for T a member of TypePackVariant: unwrap
/// the TypePackId member, then `get<T>(*tp)`.
pub fn get_type_or_pack_mut_2<T: TypePackVariantMember + 'static>(
    ty_or_tp: &TypeOrPack,
) -> *const T {
    match TypePackId::get_if(ty_or_tp) {
        Some(tp) => unsafe { crate::functions::get_type_pack::get::<T>(*tp) },
        None => core::ptr::null(),
    }
}
