use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::bound_type_pack::BoundTypePack;

impl TypeCloner {
    pub fn clone_children_bound_type_pack(&mut self, t: *mut BoundTypePack) {
        unsafe {
            (*t).boundTo = self.shallow_clone_type_pack_id((*t).boundTo);
        }
    }
}
