use core::ffi::c_void;

use crate::records::module_counters::ModuleCounters;

#[derive(Debug, Clone)]
pub struct Counters {
    pub(crate) l: *mut c_void,
    pub(crate) module_refs: alloc::vec::Vec<i32>,
    pub(crate) module_counters: alloc::vec::Vec<ModuleCounters>,
}

impl Default for Counters {
    fn default() -> Self {
        Self {
            l: core::ptr::null_mut(),
            module_refs: alloc::vec::Vec::new(),
            module_counters: alloc::vec::Vec::new(),
        }
    }
}
