use crate::records::type_cloner::TypeCloner;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;

impl TypeCloner {
    pub fn clone_children_type_function_instance_type_pack(
        &mut self,
        t: *mut TypeFunctionInstanceTypePack,
    ) {
        unsafe {
            for ty in (*t).typeArguments.iter_mut() {
                *ty = self.shallow_clone_type_id(*ty);
            }
            for tp in (*t).packArguments.iter_mut() {
                *tp = self.shallow_clone_type_pack_id(*tp);
            }
        }
    }
}
