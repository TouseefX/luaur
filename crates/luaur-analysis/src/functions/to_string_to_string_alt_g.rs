//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:127:to_string`
//! Source: `Analysis/include/Luau/ToString.h:127-131` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::records::type_pack_var::TypePackVar;
use alloc::string::String;

/// C++ `inline std::string toString(const TypePackVar& tp)`.
pub fn to_string_type_pack_var(tp: &TypePackVar) -> String {
    let mut opts = ToStringOptions::default();
    crate::functions::to_string_to_string_alt_p::to_string_type_pack_var_to_string_options(
        tp, &mut opts,
    )
}
