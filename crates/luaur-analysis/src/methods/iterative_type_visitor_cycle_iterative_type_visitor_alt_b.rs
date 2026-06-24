use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn iterative_type_visitor_cycle(&mut self) {
        self.cycle_type_pack_id(core::ptr::null());
    }

    pub fn cycle_type_pack_id(&mut self, _tp: TypePackId) {
        // Empty implementation per source: void IterativeTypeVisitor::cycle(TypePackId) {}
    }
}
