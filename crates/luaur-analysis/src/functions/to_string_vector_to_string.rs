//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1934:to_string_vector`
//! Source: `Analysis/src/ToString.cpp:1934-1944` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;

/// C++ `std::string toStringVector(const std::vector<TypeId>& types, ToStringOptions& opts)`.
pub fn to_string_vector_vector_type_id_to_string_options(
    types: &Vec<TypeId>,
    opts: &mut ToStringOptions,
) -> String {
    let mut s = String::new();
    for &ty in types.iter() {
        if !s.is_empty() {
            s.push_str(", ");
        }
        s.push_str(
            &crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
                ty, opts,
            ),
        );
    }
    s
}
