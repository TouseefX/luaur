//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ConstraintGraph.cpp:469:to_string`
//! Source: `Analysis/src/ConstraintGraph.cpp:469-487` (hand-ported)

use crate::functions::to_string_to_string::to_string_type_pack_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use alloc::format;
use alloc::string::String;

/// C++ anonymous-namespace `std::string toString(ConstraintVertex vertex)` —
/// `Luau::visit(overloaded{...}, vertex)` becomes a match over the Variant3.
pub unsafe fn to_string(vertex: ConstraintVertex) -> String {
    match vertex {
        ConstraintVertex::V0(ty) => {
            let mut opts = ToStringOptions::default();
            opts.exhaustive = true;
            format!(
                "Type {}",
                to_string_type_id_to_string_options(ty, &mut opts)
            )
        }
        ConstraintVertex::V1(tp) => {
            let mut opts = ToStringOptions::default();
            opts.exhaustive = true;
            format!(
                "Type pack {}",
                to_string_type_pack_id_to_string_options_mut(tp, opts)
            )
        }
        ConstraintVertex::V2(c) => {
            let mut opts = ToStringOptions::default();
            opts.exhaustive = true;
            format!(
                "Cons {}",
                to_string_constraint_to_string_options(&*c, &mut opts)
            )
        }
    }
}

#[allow(unused_imports)]
pub use to_string as to_string_constraint_vertex;
