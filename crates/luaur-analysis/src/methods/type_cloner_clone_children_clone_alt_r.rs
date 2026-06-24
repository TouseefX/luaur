use crate::records::type_cloner::TypeCloner;
use crate::records::union_type::UnionType;

impl TypeCloner {
    pub fn clone_children_union_type(&mut self, t: *mut UnionType) {
        unsafe {
            for ty in &mut (*t).options {
                *ty = self.shallow_clone_type_id(*ty);
            }
        }
    }
}
