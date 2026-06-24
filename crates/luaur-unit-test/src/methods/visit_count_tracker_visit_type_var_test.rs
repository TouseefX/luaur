//! @interface-stub
use crate::records::visit_count_tracker::VisitCountTracker;
use luaur_analysis::type_aliases::type_id::TypeId;

impl VisitCountTracker {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        self.visit_type(ty)
    }
}
