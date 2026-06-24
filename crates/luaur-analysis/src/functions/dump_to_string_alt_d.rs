//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1847:dump`
//! Source: `Analysis/src/ToString.cpp:1847-1854` (hand-ported)

use crate::type_aliases::type_id::TypeId;
use alloc::string::{String, ToString};

/// C++ `std::string dump(const std::optional<TypeId>& ty)`.
pub fn dump_optional_type_id(ty: &Option<TypeId>) -> String {
    if let Some(ty) = ty {
        return crate::functions::dump_to_string::dump_type_id(*ty);
    }

    std::println!("nullopt");
    "nullopt".to_string()
}
