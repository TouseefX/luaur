extern crate alloc;

use crate::records::node::Node;
use core::ffi::c_void;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EnumContext {
    pub nodes: luaur_common::records::dense_hash_map::DenseHashMap<*mut c_void, Node>,
    pub edges: luaur_common::records::dense_hash_map::DenseHashMap<*mut c_void, *mut c_void>,
    pub seen_target_string: bool,
}
