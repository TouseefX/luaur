use crate::records::tarjan::Tarjan;
use crate::type_aliases::type_pack_id::TypePackId;

impl Tarjan {
    pub fn visit_child_type_pack_id(&mut self, tp: TypePackId) {
        let tp = unsafe { (*self.log).follow_type_pack_id(tp) };

        self.edges_ty.push(core::ptr::null_mut());
        self.edges_tp.push(tp);
    }
}
