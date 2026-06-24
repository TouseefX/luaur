use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;

impl TypeFunctionCloner {
    pub fn clone_children_type_function_variadic_type_pack_type_function_variadic_type_pack(
        &mut self,
        v1: *mut TypeFunctionVariadicTypePack,
        v2: *mut TypeFunctionVariadicTypePack,
    ) {
        let source_type = unsafe { (*v1).type_id };
        let target_type = self.shallow_clone_type_function_type_id(source_type);
        unsafe { (*v2).type_id = target_type };
    }
}
