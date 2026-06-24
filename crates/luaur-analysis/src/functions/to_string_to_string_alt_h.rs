//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:169:to_string`
//! Source: `Analysis/include/Luau/ToString.h:169-173` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_or_pack::TypeOrPack;
use alloc::string::String;

/// C++ `inline std::string toString(const TypeOrPack& tyOrTp)`.
pub fn to_string_type_or_pack(ty_or_tp: &TypeOrPack) -> String {
    let mut opts = ToStringOptions::default();
    crate::functions::to_string_to_string_alt_u::to_string_type_or_pack_to_string_options(
        ty_or_tp, &mut opts,
    )
}
