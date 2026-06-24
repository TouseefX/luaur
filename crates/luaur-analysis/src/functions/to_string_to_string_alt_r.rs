//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2060:to_string`
//! Source: `Analysis/src/ToString.cpp:2060-2063` (hand-ported)

use crate::records::constraint::Constraint;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `std::string toString(const Constraint& constraint)`.
pub fn to_string_constraint(constraint: &Constraint) -> String {
    let mut opts = ToStringOptions::default();
    crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options(
        constraint, &mut opts,
    )
}
