//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:515:type_visitor_type_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:515-518` (hand-ported)

use crate::records::type_visitor::TypeVisitor;
use alloc::string::String;

impl TypeVisitor {
    /// C++ `explicit TypeVisitor(const std::string visitorName, bool skipBoundTypes)`.
    /// Pinned-name alias of [`TypeVisitor::new`].
    pub fn type_visitor(visitor_name: String, skip_bound_types: bool) -> Self {
        Self::new(visitor_name, skip_bound_types)
    }
}
