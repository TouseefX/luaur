//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TxnLog.cpp:25:dump`
//! Source: `Analysis/src/TxnLog.cpp:25-39` (hand-ported)

use crate::functions::to_string_to_string_alt_o::to_string_type_item_to_string_options;
use crate::functions::to_string_txn_log::NULL_PENDING_RESULT;
use crate::records::pending_type::PendingType;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `std::string dump(PendingType* pending)`.
pub unsafe fn dump(pending: *mut PendingType) -> String {
    if pending.is_null() {
        println!("{}", NULL_PENDING_RESULT);
        return String::from(NULL_PENDING_RESULT);
    }

    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    opts.function_type_arguments = true;
    let result = to_string_type_item_to_string_options(&(*pending).pending, &mut opts);
    println!("{}", result);
    result
}
