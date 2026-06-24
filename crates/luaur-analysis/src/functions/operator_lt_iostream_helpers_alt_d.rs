use crate::enums::reason::Reason;
use alloc::string::String;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt_iostream_helpers_alt_d(stream: &mut String, reason: Reason) {
    match reason {
        Reason::PropertyNarrowed => {
            let _ = write!(stream, "PropertyNarrowed");
        }
        #[allow(unreachable_patterns)]
        _ => {
            let _ = write!(stream, "UnknownReason");
        }
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_lt_iostream_helpers_alt_d as operator_lt_ostream_cannot_assign_to_never_reason;
