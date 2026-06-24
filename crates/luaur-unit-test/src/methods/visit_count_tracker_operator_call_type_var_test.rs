//! @interface-stub
use crate::records::visit_count_tracker::VisitCountTracker;
use luaur_analysis::type_aliases::type_id::TypeId;

impl VisitCountTracker {
    pub fn operator_call<T>(&mut self, ty: TypeId, _t: &T) -> bool {
        self.visit_type_id(ty)
    }
}
