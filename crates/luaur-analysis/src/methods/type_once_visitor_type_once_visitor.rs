//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:524:type_once_visitor_type_once_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:524-527` (hand-ported)

use crate::records::type_once_visitor::TypeOnceVisitor;
use alloc::string::String;

impl TypeOnceVisitor {
    /// C++ `explicit TypeOnceVisitor(const std::string visitorName, bool skipBoundTypes)`.
    /// Pinned-name alias of [`TypeOnceVisitor::new`].
    pub fn type_once_visitor(visitor_name: String, skip_bound_types: bool) -> Self {
        Self::new(visitor_name, skip_bound_types)
    }
}
