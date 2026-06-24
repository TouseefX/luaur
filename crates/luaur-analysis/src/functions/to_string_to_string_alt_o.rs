//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1732:to_string`
//! Source: `Analysis/src/ToString.cpp:1732-1735` (hand-ported)

use crate::records::r#type::Type;
use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

/// C++ `std::string toString(const Type& tv, ToStringOptions& opts)`.
pub fn to_string_type_item_to_string_options(tv: &Type, opts: &mut ToStringOptions) -> String {
    crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
        tv as *const Type as TypeId,
        opts,
    )
}
