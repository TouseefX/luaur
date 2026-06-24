use crate::records::metatable_type::MetatableType;
use crate::records::type_cloner::TypeCloner;

impl TypeCloner {
    pub fn clone_children_metatable_type(&mut self, t: *mut MetatableType) {
        unsafe {
            (*t).table = self.shallow_clone_type_id((*t).table);
            (*t).metatable = self.shallow_clone_type_id((*t).metatable);
        }
    }
}
