use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn lazy_type_lazy_type(&self) -> Self {
        let unwrap_fn = self.unwrap;
        let unwrapped_ptr = self.unwrapped;
        let mut cloned =
            LazyType::lazy_type_function_void_lazy_type_item(unwrap_fn.unwrap_or(|_| {}));
        cloned.unwrapped = unwrapped_ptr;
        cloned
    }
}
