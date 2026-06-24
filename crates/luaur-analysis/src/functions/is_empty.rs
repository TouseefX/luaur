//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:369:is_empty`
//! Source: `Analysis/src/TypePack.cpp:369-378` (hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;

pub fn is_empty(tp: TypePackId) -> bool {
    unsafe {
        let tp = follow_type_pack_id(tp);
        if let Some(tpp) = TypePack::get_if(&(*tp).ty) {
            return tpp.head.is_empty()
                && match tpp.tail {
                    Some(tail) => is_empty(tail),
                    None => true,
                };
        }

        false
    }
}
