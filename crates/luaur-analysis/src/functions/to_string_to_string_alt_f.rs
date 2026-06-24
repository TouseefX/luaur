//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:121:to_string`
//! Source: `Analysis/include/Luau/ToString.h:121-125` (hand-ported)

use crate::records::r#type::Type;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `inline std::string toString(const Type& tv)`.
pub fn to_string_type_item(tv: &Type) -> String {
    let mut opts = ToStringOptions::default();
    crate::functions::to_string_to_string_alt_o::to_string_type_item_to_string_options(
        tv, &mut opts,
    )
}
