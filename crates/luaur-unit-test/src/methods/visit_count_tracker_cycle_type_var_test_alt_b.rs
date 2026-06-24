use crate::records::visit_count_tracker::VisitCountTracker;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl VisitCountTracker {
    pub fn cycle_type_pack_id(&mut self, _tp: TypePackId) {
        // The C++ method body is empty.
    }
}
