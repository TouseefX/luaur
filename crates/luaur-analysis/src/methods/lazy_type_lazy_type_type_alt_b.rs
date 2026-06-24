use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn lazy_type_function_void_lazy_type_item(unwrap: fn(&mut LazyType)) -> Self {
        Self {
            unwrap: Some(unwrap),
            unwrapped: core::ptr::null(),
        }
    }
}
