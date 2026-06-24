//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2151:dump`
//! Source: `Analysis/src/ToString.cpp:2151-2159` (hand-ported)

use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_or_pack::TypeOrPack;
use alloc::string::String;

/// C++ `std::string dump(const TypeOrPack& tyOrTp)`.
pub fn dump_type_or_pack(ty_or_tp: &TypeOrPack) -> String {
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    opts.function_type_arguments = true;
    let s = crate::functions::to_string_to_string_alt_u::to_string_type_or_pack_to_string_options(
        ty_or_tp, &mut opts,
    );
    std::println!("{}", s);
    s
}
