//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:225:getTail`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:225-243, hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn get_tail(mut tp: TypePackId) -> TypePackId {
    let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(core::ptr::null());
    unsafe {
        while !tp.is_null() {
            tp = follow_type_pack_id(tp);

            if seen.contains(&tp) {
                break;
            }
            seen.insert(tp);

            let pack = get_type_pack_id::<TypePack>(tp);
            if !pack.is_null() {
                if let Some(tail) = (*pack).tail {
                    tp = tail;
                    continue;
                }
            }
            break;
        }

        follow_type_pack_id(tp)
    }
}
