use crate::records::serialized_function_scope::SerializedFunctionScope;
use crate::records::type_function_function_type::TypeFunctionFunctionType;

impl SerializedFunctionScope {
    pub fn serialized_function_scope_usize_type_function_function_type(
        &mut self,
        old_queue_size: usize,
        function: *mut TypeFunctionFunctionType,
    ) {
        self.old_queue_size = old_queue_size;
        self.function = function;
    }
}
