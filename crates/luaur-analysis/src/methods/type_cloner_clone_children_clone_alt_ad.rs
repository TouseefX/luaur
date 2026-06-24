use crate::records::type_cloner::TypeCloner;
use crate::records::variadic_type_pack::VariadicTypePack;

impl TypeCloner {
    pub fn clone_children_variadic_type_pack(&mut self, t: *mut VariadicTypePack) {
        unsafe {
            (*t).ty = self.shallow_clone_type_id((*t).ty);
        }
    }
}
