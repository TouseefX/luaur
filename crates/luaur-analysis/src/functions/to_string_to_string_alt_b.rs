//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:92:to_string`
//! Source: `Analysis/include/Luau/ToString.h:92-96` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

/// C++ `inline std::string toString(TypeId ty, ToStringOptions&& opts)`.
pub fn to_string_type_id_to_string_options_mut(ty: TypeId, mut opts: ToStringOptions) -> String {
    crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(ty, &mut opts)
}
