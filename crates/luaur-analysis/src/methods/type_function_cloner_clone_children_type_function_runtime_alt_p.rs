use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::records::type_function_type_pack::TypeFunctionTypePack;

impl TypeFunctionCloner {
    pub fn clone_children_type_function_type_pack_type_function_type_pack(
        &mut self,
        t1: *mut TypeFunctionTypePack,
        t2: *mut TypeFunctionTypePack,
    ) {
        let head = unsafe { &(*t1).head };
        let target_head = unsafe { &mut (*t2).head };
        for ty in head.iter() {
            target_head.push(self.shallow_clone_type_function_type_id(*ty));
        }

        let tail = unsafe { (*t1).tail };
        if let Some(t) = tail {
            let cloned_tail = self.shallow_clone_type_function_type_pack_id(t);
            unsafe { (*t2).tail = Some(cloned_tail) };
        }
    }
}
