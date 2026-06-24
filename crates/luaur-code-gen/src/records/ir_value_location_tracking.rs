use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrValueLocationTracking {
    pub function: *mut IrFunction,
    pub vm_reg_value: [u32; 256],
    pub vm_reg_dependent: [u32; 256],
    pub max_reg: i32,
    pub restore_callback_ctx: *mut core::ffi::c_void,
    pub restore_callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut IrInst)>,
}
