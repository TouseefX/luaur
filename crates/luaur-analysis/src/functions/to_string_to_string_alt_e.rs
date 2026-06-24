//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:111:to_string`
//! Source: `Analysis/include/Luau/ToString.h:111-114` (hand-ported)

use crate::records::constraint::Constraint;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `inline std::string toString(const Constraint& c, ToStringOptions&& opts)`.
pub fn to_string_constraint_to_string_options_mut(
    c: &Constraint,
    mut opts: ToStringOptions,
) -> String {
    crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options(
        c, &mut opts,
    )
}
