//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1856:dump`
//! Source: `Analysis/src/ToString.cpp:1856-1861` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

/// C++ `std::string dump(TypePackId ty)`.
pub fn dump_type_pack_id(ty: TypePackId) -> String {
    let s = crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options(
        ty,
        dump_options(),
    );
    std::println!("{}", s);
    s
}
