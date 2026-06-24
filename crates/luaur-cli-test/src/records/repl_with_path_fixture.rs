extern crate alloc;

use alloc::string::String;

pub struct ReplWithPathFixture {
    pub(crate) lua_state: *mut core::ffi::c_void,
    pub(crate) l: *mut core::ffi::c_void,
    pub(crate) pretty_print_source: String,
}
