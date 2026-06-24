//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1863:dump`
//! Source: `Analysis/src/ToString.cpp:1863-1870` (hand-ported)

use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::{String, ToString};

/// C++ `std::string dump(const std::optional<TypePackId>& ty)`.
pub fn dump_optional_type_pack_id(ty: &Option<TypePackId>) -> String {
    if let Some(tp) = ty {
        return crate::functions::dump_to_string_alt_e::dump_type_pack_id(*tp);
    }

    std::println!("nullopt");
    "nullopt".to_string()
}
