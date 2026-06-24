extern crate alloc;

use crate::records::cfg_info::CfgInfo;
use crate::records::ir_block::IrBlock;
use crate::records::ir_const::IrConst;
use crate::records::vm_exit_sync_info::VmExitSyncInfo;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[allow(non_camel_case_types)]
pub struct ir_to_string_context<'a> {
    pub result: &'a mut String,
    pub blocks: &'a Vec<IrBlock>,
    pub constants: &'a Vec<IrConst>,
    pub cfg: &'a CfgInfo,
    pub vm_exit_info: &'a DenseHashMap<u32, VmExitSyncInfo>,
    pub proto: *mut core::ffi::c_void, // Proto is an opaque struct in this context
}

pub type IrToStringContext<'a> = ir_to_string_context<'a>;
