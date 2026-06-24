//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:440:containsNever`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:440-459, hand-ported)

use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::never_type::NeverType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn contains_never(tp: TypePackId) -> bool {
    unsafe {
        let mut it = begin_type_pack_id(tp);
        let end_it = end(tp);

        while it.operator_ne(&end_it) {
            if !get_type_id::<NeverType>(follow_type_id(*it.operator_deref())).is_null() {
                return true;
            }
            it.operator_inc();
        }

        if let Some(tail) = it.tail() {
            let vtp = get_type_pack_id::<VariadicTypePack>(tail);
            if !vtp.is_null() && !get_type_id::<NeverType>(follow_type_id((*vtp).ty)).is_null() {
                return true;
            }
        }

        false
    }
}
