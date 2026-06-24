extern crate alloc;

use luaur_vm::records::proto::Proto;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssertInlinerData {
    pub proto: *mut Proto,
    pub target: *mut Proto,
    pub pc: u32,
    pub called: bool,
}
