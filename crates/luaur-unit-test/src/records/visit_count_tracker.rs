use alloc::collections::BTreeMap;
use alloc::string::String;
use core::ffi::c_void;

use luaur_analysis::records::type_once_visitor::TypeOnceVisitor;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct VisitCountTracker {
    pub(crate) base: TypeOnceVisitor,
    pub(crate) ty_visits: BTreeMap<*const c_void, u32>,
    pub(crate) tp_visits: BTreeMap<*const c_void, u32>,
}

impl VisitCountTracker {
    pub fn visit_count_tracker_visit_count_tracker(&mut self) {}

    pub fn cycle(&mut self, _ty: TypeId) {}

    pub fn cycle_type_pack(&mut self, _tp: TypePackId) {}

    pub fn visit_type(&mut self, ty: TypeId) -> bool {
        let key = ty as *const c_void;
        let entry = self.ty_visits.entry(key).or_insert(0);
        *entry += 1;
        true
    }

    pub fn visit_type_pack(&mut self, tp: TypePackId) -> bool {
        let key = tp as *const c_void;
        let entry = self.tp_visits.entry(key).or_insert(0);
        *entry += 1;
        true
    }

    pub fn operator_type(&mut self, ty: TypeId) -> bool {
        self.visit_type(ty)
    }

    pub fn operator_type_pack(&mut self, tp: TypePackId) -> bool {
        self.visit_type_pack(tp)
    }

    pub fn visit(&mut self, ty: TypeId) -> bool {
        self.visit_type(ty)
    }

    pub fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        self.visit_type_pack(tp)
    }
}
