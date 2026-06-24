use crate::records::tarjan::Tarjan;
use crate::type_aliases::type_id::TypeId;

impl Tarjan {
    pub fn visit_child_type_id(&mut self, ty: TypeId) {
        let ty = unsafe { (*self.log).follow_type_id(ty) };

        self.edges_ty.push(ty);
        self.edges_tp.push(core::ptr::null_mut());
    }
}
