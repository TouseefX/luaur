//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:81:generic_type_visitor_generic_type_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:81` (hand-ported)

use crate::records::generic_type_visitor::GenericTypeVisitor;
use alloc::string::String;

impl<S: Default> GenericTypeVisitor<S> {
    /// C++ `GenericTypeVisitor() = default;`
    pub fn generic_type_visitor() -> Self {
        Self {
            visitor_name: String::new(),
            seen: S::default(),
            skip_bound_types: false,
            recursion_counter: 0,
            type_function_depth: 0,
        }
    }
}
