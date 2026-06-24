//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2065:dump`
//! Source: `Analysis/src/ToString.cpp:2065-2073` (hand-ported)

use crate::records::constraint::Constraint;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `std::string dump(const Constraint& c)`.
pub fn dump_constraint(c: &Constraint) -> String {
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    opts.function_type_arguments = true;
    let s = crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options(
        c, &mut opts,
    );
    std::println!("{}", s);
    s
}
