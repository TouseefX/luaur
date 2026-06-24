use crate::records::serialized_function_scope::SerializedFunctionScope;

impl SerializedFunctionScope {
    pub fn serialized_function_scope_usize(&mut self, old_queue_size: usize) {
        self.old_queue_size = old_queue_size;
        self.function = std::ptr::null_mut();
    }
}
