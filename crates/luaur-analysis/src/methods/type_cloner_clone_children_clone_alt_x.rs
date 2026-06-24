use crate::records::type_cloner::TypeCloner;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;

impl TypeCloner {
    pub fn clone_children_type_function_instance_type(&mut self, t: *mut TypeFunctionInstanceType) {
        unsafe {
            for ty in (*t).type_arguments.iter_mut() {
                *ty = self.shallow_clone_type_id(*ty);
            }
            for tp in (*t).pack_arguments.iter_mut() {
                *tp = self.shallow_clone_type_pack_id(*tp);
            }
        }
    }
}
