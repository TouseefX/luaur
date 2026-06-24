extern crate alloc;

use crate::records::module_counters::ModuleCounters;
use alloc::vec::Vec;
use core::ffi::c_int;

#[allow(non_camel_case_types)]
pub type lua_State = core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct Counters {
    pub(crate) L: *mut lua_State,
    pub(crate) module_refs: Vec<c_int>,
    pub(crate) module_counters: Vec<ModuleCounters>,
}

#[allow(non_upper_case_globals)]
pub(crate) static mut gCounters: Counters = Counters {
    L: core::ptr::null_mut(),
    module_refs: Vec::new(),
    module_counters: Vec::new(),
};
