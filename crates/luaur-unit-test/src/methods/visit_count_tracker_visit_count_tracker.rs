use crate::records::visit_count_tracker::VisitCountTracker;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::ffi::c_void;
use luaur_analysis::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use luaur_analysis::records::type_once_visitor::TypeOnceVisitor;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl VisitCountTracker {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("VisitCountTracker"), true),
            ty_visits: BTreeMap::new(),
            tp_visits: BTreeMap::new(),
        }
    }

    pub fn traverse(&mut self, ty: TypeId) {
        GenericTypeVisitorTrait::traverse_type_id(self, ty);
    }
}

impl GenericTypeVisitorTrait for VisitCountTracker {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn cycle_type_id(&mut self, ty: TypeId) {
        VisitCountTracker::cycle_type_id(self, ty);
    }

    fn cycle_type_pack_id(&mut self, tp: TypePackId) {
        VisitCountTracker::cycle_type_pack_id(self, tp);
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        VisitCountTracker::visit_type_id(self, ty)
    }

    fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        VisitCountTracker::visit_type_pack_id(self, tp)
    }
}
