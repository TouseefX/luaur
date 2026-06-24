//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:87:to_string`
//! Source: `Analysis/include/Luau/ToString.h:87-91` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

/// C++ `inline std::string toString(TypePackId ty, ToStringOptions&& opts)`.
pub fn to_string_type_pack_id_to_string_options_mut(
    ty: TypePackId,
    mut opts: ToStringOptions,
) -> String {
    crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
        ty, &mut opts,
    )
}
