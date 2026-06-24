//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TxnLog.cpp:17:to_string`
//! Source: `Analysis/src/TxnLog.cpp:15-23` (hand-ported)

use crate::functions::to_string_to_string_alt_f::to_string_type_item;
use crate::records::pending_type::PendingType;
use alloc::string::String;

/// C++ `const std::string nullPendingResult = "<nullptr>";` (TxnLog.cpp:15).
pub const NULL_PENDING_RESULT: &str = "<nullptr>";

/// C++ `std::string toString(PendingType* pending)`.
pub unsafe fn to_string_pending_type(pending: *mut PendingType) -> String {
    if pending.is_null() {
        return String::from(NULL_PENDING_RESULT);
    }

    to_string_type_item(&(*pending).pending)
}

#[allow(unused_imports)]
pub use to_string_pending_type as to_string;
