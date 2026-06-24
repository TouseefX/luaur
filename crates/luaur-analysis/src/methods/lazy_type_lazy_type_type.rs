use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn lazy_type() -> Self {
        Self {
            unwrap: None,
            unwrapped: core::ptr::null(),
        }
    }
}
