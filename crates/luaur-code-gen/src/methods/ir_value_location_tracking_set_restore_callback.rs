use crate::records::ir_inst::IrInst;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use core::ffi::c_void;

impl IrValueLocationTracking {
    pub fn set_restore_callback(
        &mut self,
        context: *mut c_void,
        callback: Option<unsafe extern "C" fn(*mut c_void, *mut IrInst)>,
    ) {
        self.restore_callback_ctx = context;
        self.restore_callback = callback;
    }
}
