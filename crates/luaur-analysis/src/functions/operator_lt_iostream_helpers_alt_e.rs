use crate::enums::interesting_edge_case::InterestingEdgeCase;
use alloc::string::String;
use core::fmt::Write;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[allow(non_snake_case)]
pub fn operator_lt_iostream_helpers_alt_e(stream: &mut String, edge_case: InterestingEdgeCase) {
    match edge_case {
        InterestingEdgeCase::None => {
            let _ = write!(stream, "None");
        }
        InterestingEdgeCase::MetatableCall => {
            let _ = write!(stream, "MetatableCall");
        }
        InterestingEdgeCase::Intersection => {
            let _ = write!(stream, "Intersection");
        }
        _ => {
            LUAU_ASSERT!(false);
            let _ = write!(stream, "Unknown");
        }
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_lt_iostream_helpers_alt_e as operator_lt_ostream_instantiate_generics_on_non_function_interesting_edge_case;
