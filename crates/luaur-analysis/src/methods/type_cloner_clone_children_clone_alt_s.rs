use crate::records::intersection_type::IntersectionType;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_id::TypeId;

impl TypeCloner {
    pub fn clone_children_intersection_type(&mut self, t: *mut IntersectionType) {
        let parts = unsafe { &mut (*t).parts };
        for ty in parts.iter_mut() {
            *ty = self.shallow_clone_type_id(*ty);
        }
    }
}
