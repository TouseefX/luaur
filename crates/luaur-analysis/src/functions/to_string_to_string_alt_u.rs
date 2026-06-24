//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2141:to_string`
//! Source: `Analysis/src/ToString.cpp:2141-2149` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

/// C++ `std::string toString(const TypeOrPack& tyOrTp, ToStringOptions& opts)`.
pub fn to_string_type_or_pack_to_string_options(
    ty_or_tp: &TypeOrPack,
    opts: &mut ToStringOptions,
) -> String {
    if let Some(ty) = TypeId::get_if(ty_or_tp) {
        crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(*ty, opts)
    } else if let Some(tp) = TypePackId::get_if(ty_or_tp) {
        crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
            *tp, opts,
        )
    } else {
        unreachable!("LUAU_UNREACHABLE: TypeOrPack has exactly two members")
    }
}
