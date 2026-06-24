//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:330:size`
//! Source: `Analysis/src/TypePack.cpp:330-340` (hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;

/// C++ `size_t size(const TypePack& tp, TxnLog* log = nullptr)`.
pub fn size(tp: &TypePack, log: *mut TxnLog) -> usize {
    unsafe {
        let mut result = tp.head.len();
        if let Some(tp_tail) = tp.tail {
            let followed = if !log.is_null() {
                (*log).follow_type_pack_id(tp_tail)
            } else {
                follow_type_pack_id(tp_tail)
            };
            if let Some(tail) = TypePack::get_if(&(*followed).ty) {
                result += size(tail, log);
            }
        }
        result
    }
}
