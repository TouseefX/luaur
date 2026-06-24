use crate::records::type_cloner::TypeCloner;
use crate::records::type_pack::TypePack;

impl TypeCloner {
    pub fn clone_children_type_pack(&mut self, t: *mut TypePack) {
        unsafe {
            for ty in (*t).head.iter_mut() {
                *ty = self.shallow_clone_type_id(*ty);
            }
            if let Some(tail) = (*t).tail {
                (*t).tail = Some(self.shallow_clone_type_pack_id(tail));
            }
        }
    }
}
