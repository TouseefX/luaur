//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1727:to_string`
//! Source: `Analysis/src/ToString.cpp:1727-1730` (hand-ported)

use crate::functions::to_string_detailed_to_string_alt_b::to_string_detailed_type_pack_id_to_string_options;
use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

/// C++ `std::string toString(TypePackId ty, ToStringOptions& opts)`.
pub fn to_string_type_pack_id_to_string_options(
    ty: TypePackId,
    opts: &mut ToStringOptions,
) -> String {
    to_string_detailed_type_pack_id_to_string_options(ty, opts).name
}
