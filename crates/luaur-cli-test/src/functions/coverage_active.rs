use std::collections::HashMap;

#[allow(non_camel_case_types)]
pub type lua_State = core::ffi::c_void;

#[allow(non_camel_case_types)]
pub struct Coverage {
    pub L: *mut lua_State,
    pub results: Option<HashMap<String, Vec<i32>>>,
}

#[allow(non_upper_case_globals)]
pub static mut gCoverage: Coverage = Coverage {
    L: std::ptr::null_mut(),
    results: None,
};

pub fn coverage_active() -> bool {
    unsafe { !gCoverage.L.is_null() }
}
