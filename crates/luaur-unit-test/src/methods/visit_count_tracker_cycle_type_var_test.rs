use crate::records::visit_count_tracker::VisitCountTracker;
use luaur_analysis::type_aliases::type_id::TypeId;

impl VisitCountTracker {
    pub fn cycle_type_id(&mut self, _ty: TypeId) {
        // Empty implementation: the C++ method body is empty
    }
}
