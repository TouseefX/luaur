use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn lazy_type_lazy_type_mut() -> Self {
        LazyType {
            unwrap: None,
            unwrapped: core::ptr::null(),
        }
    }
}
