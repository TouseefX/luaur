//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1872:dump`
//! Source: `Analysis/src/ToString.cpp:1872-1875` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;

/// C++ `std::string dump(const std::vector<TypeId>& types)`.
pub fn dump_vector_type_id(types: &Vec<TypeId>) -> String {
    crate::functions::to_string_vector_to_string::to_string_vector_vector_type_id_to_string_options(
        types,
        dump_options(),
    )
}
