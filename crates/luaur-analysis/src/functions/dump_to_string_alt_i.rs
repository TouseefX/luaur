//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1882:dump`
//! Source: `Analysis/src/ToString.cpp:1882-1894` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;

/// C++ `std::string dump(DenseHashMap<TypeId, TypeId>& types)`.
pub fn dump_dense_hash_map_type_id_type_id(types: &mut DenseHashMap<TypeId, TypeId>) -> String {
    let mut s = String::from("{");
    let opts = dump_options();
    let pairs: alloc::vec::Vec<(TypeId, TypeId)> = types.iter().map(|(k, v)| (*k, *v)).collect();
    for (key, value) in pairs {
        if s.len() > 1 {
            s.push_str(", ");
        }
        s.push_str(&format!(
            "{} : {}",
            crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
                key, opts
            ),
            crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
                value,
                dump_options()
            ),
        ));
    }
    s.push('}');
    s
}
