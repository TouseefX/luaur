extern crate alloc;

use alloc::vec::Vec;
use core::ffi::c_int;

#[allow(non_camel_case_types)]
pub type lua_State = core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct Coverage {
    pub(crate) L: *mut lua_State,
    pub(crate) functions: Vec<c_int>,
}

#[allow(non_upper_case_globals)]
pub(crate) static mut gCoverage: Coverage = Coverage {
    L: core::ptr::null_mut(),
    functions: Vec::new(),
};
