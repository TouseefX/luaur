//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1946:to_string_vector`
//! Source: `Analysis/src/ToString.cpp:1946-1956` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;

/// C++ `std::string toStringVector(const std::vector<TypePackId>& typePacks, ToStringOptions& opts)`.
pub fn to_string_vector_vector_type_pack_id_to_string_options(
    type_packs: &Vec<TypePackId>,
    opts: &mut ToStringOptions,
) -> String {
    let mut s = String::new();
    for &type_pack in type_packs.iter() {
        if !s.is_empty() {
            s.push_str(", ");
        }
        s.push_str(
            &crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
                type_pack, opts,
            ),
        );
    }
    s
}
