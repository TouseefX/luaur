//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1896:dump`
//! Source: `Analysis/src/ToString.cpp:1896-1908` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::format;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;

/// C++ `std::string dump(DenseHashMap<TypePackId, TypePackId>& types)`.
pub fn dump_dense_hash_map_type_pack_id_type_pack_id(
    types: &mut DenseHashMap<TypePackId, TypePackId>,
) -> String {
    let mut s = String::from("{");
    let pairs: alloc::vec::Vec<(TypePackId, TypePackId)> =
        types.iter().map(|(k, v)| (*k, *v)).collect();
    for (key, value) in pairs {
        if s.len() > 1 {
            s.push_str(", ");
        }
        s.push_str(&format!(
            "{} : {}",
            crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
                key,
                dump_options()
            ),
            crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
                value,
                dump_options()
            ),
        ));
    }
    s.push('}');
    s
}
