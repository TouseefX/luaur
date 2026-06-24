use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::bound_type::BoundType;

impl TypeCloner {
    pub fn clone_children_bound_type(&mut self, t: *mut BoundType) {
        unsafe {
            (*t).boundTo = self.shallow_clone_type_id((*t).boundTo);
        }
    }
}
