//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/VisitType.h:513:type_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:513-519` (hand-ported)
//!
//! C++ `struct TypeVisitor : GenericTypeVisitor<std::unordered_set<void*>>`.
//! Visits each type under a given type; the same type may be visited multiple
//! times via distinct paths (the seen set forgets on unsee).

use crate::records::generic_type_visitor::GenericTypeVisitor;
use alloc::string::String;
use core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct TypeVisitor {
    pub base: GenericTypeVisitor<std::collections::HashSet<*mut c_void>>,
}

impl TypeVisitor {
    /// C++ `explicit TypeVisitor(const std::string visitorName, bool skipBoundTypes)`.
    pub fn new(visitor_name: String, skip_bound_types: bool) -> Self {
        Self {
            base: GenericTypeVisitor::generic_type_visitor_string_set_bool(
                visitor_name,
                std::collections::HashSet::new(),
                skip_bound_types,
            ),
        }
    }
}
