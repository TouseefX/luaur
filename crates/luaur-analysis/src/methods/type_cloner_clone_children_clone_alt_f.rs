use crate::records::free_type::FreeType;
use crate::records::type_cloner::TypeCloner;

impl TypeCloner {
    pub fn clone_children_free_type(&mut self, t: *mut FreeType) {
        unsafe {
            if !(*t).lower_bound.is_null() {
                (*t).lower_bound = self.shallow_clone_type_id((*t).lower_bound);
            }
            if !(*t).upper_bound.is_null() {
                (*t).upper_bound = self.shallow_clone_type_id((*t).upper_bound);
            }
        }
    }
}
