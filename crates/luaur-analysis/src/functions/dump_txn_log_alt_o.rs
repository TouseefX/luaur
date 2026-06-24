//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TxnLog.cpp:49:dump`
//! Source: `Analysis/src/TxnLog.cpp:49-63` (hand-ported)

use crate::functions::to_string_to_string_alt_p::to_string_type_pack_var_to_string_options;
use crate::functions::to_string_txn_log::NULL_PENDING_RESULT;
use crate::records::pending_type_pack::PendingTypePack;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `std::string dump(PendingTypePack* pending)`.
pub unsafe fn dump(pending: *mut PendingTypePack) -> String {
    if pending.is_null() {
        println!("{}", NULL_PENDING_RESULT);
        return String::from(NULL_PENDING_RESULT);
    }

    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    opts.function_type_arguments = true;
    let result = to_string_type_pack_var_to_string_options(&(*pending).pending, &mut opts);
    println!("{}", result);
    result
}
