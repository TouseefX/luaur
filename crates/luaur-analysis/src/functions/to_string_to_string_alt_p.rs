//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1737:to_string`
//! Source: `Analysis/src/ToString.cpp:1737-1740` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

/// C++ `std::string toString(const TypePackVar& tp, ToStringOptions& opts)`.
pub fn to_string_type_pack_var_to_string_options(
    tp: &TypePackVar,
    opts: &mut ToStringOptions,
) -> String {
    crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
        tp as *const TypePackVar as TypePackId,
        opts,
    )
}
