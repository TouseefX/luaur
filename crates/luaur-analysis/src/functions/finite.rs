//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:317:finite`
//! Source: `Analysis/src/TypePack.cpp:317-328` (hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;

/// C++ `bool finite(TypePackId tp, TxnLog* log = nullptr)`.
pub fn finite(tp: TypePackId, log: *mut TxnLog) -> bool {
    unsafe {
        let tp = if !log.is_null() {
            (*log).follow_type_pack_id(tp)
        } else {
            follow_type_pack_id(tp)
        };

        if let Some(pack) = TypePack::get_if(&(*tp).ty) {
            return match pack.tail {
                Some(tail) => finite(tail, log),
                None => true,
            };
        }

        if VariadicTypePack::get_if(&(*tp).ty).is_some() {
            return false;
        }

        true
    }
}
