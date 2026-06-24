//! @interface-stub
use crate::records::visit_count_tracker::VisitCountTracker;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl VisitCountTracker {
    pub fn operator_call_2<T>(&mut self, tp: TypePackId, _t: &T) -> bool {
        self.visit_type_pack_id(tp)
    }
}
