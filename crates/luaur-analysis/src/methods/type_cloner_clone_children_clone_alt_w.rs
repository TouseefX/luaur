use crate::records::negation_type::NegationType;
use crate::records::type_cloner::TypeCloner;

impl TypeCloner {
    pub fn clone_children_negation_type(&mut self, t: *mut NegationType) {
        unsafe {
            (*t).ty = self.shallow_clone_type_id((*t).ty);
        }
    }
}
