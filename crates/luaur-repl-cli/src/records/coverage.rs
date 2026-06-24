use core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct Coverage {
    pub(crate) l: *mut c_void,
    pub(crate) functions: alloc::vec::Vec<i32>,
}

impl Default for Coverage {
    fn default() -> Self {
        Self {
            l: core::ptr::null_mut(),
            functions: alloc::vec::Vec::new(),
        }
    }
}
