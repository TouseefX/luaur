//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TxnLog.cpp:41:to_string`
//! Source: `Analysis/src/TxnLog.cpp:41-47` (hand-ported)

use crate::functions::to_string_to_string_alt_g::to_string_type_pack_var;
use crate::functions::to_string_txn_log::NULL_PENDING_RESULT;
use crate::records::pending_type_pack::PendingTypePack;
use alloc::string::String;

/// C++ `std::string toString(PendingTypePack* pending)`.
pub unsafe fn to_string_pending_type_pack(pending: *mut PendingTypePack) -> String {
    if pending.is_null() {
        return String::from(NULL_PENDING_RESULT);
    }

    to_string_type_pack_var(&(*pending).pending)
}

#[allow(unused_imports)]
pub use to_string_pending_type_pack as to_string;
