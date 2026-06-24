//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1840:dump`
//! Source: `Analysis/src/ToString.cpp:1840-1845` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

/// C++ `std::string dump(TypeId ty)`.
pub fn dump_type_id(ty: TypeId) -> String {
    let s = crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
        ty,
        dump_options(),
    );
    std::println!("{}", s);
    s
}
