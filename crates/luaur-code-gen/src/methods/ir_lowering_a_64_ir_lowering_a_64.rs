use crate::records::ir_inst::IrInst;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use core::ffi::c_void;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_ir_lowering_a_64(&mut self) {
        self.exit_handler_map = luaur_common::records::dense_hash_map::DenseHashMap::new(!0u32);

        let self_ptr = self as *mut IrLoweringA64 as *mut c_void;
        self.value_tracker
            .set_restore_callback(self_ptr, Some(Self::restore_callback_shim));
    }

    unsafe extern "C" fn restore_callback_shim(context: *mut c_void, inst: *mut IrInst) {
        let self_ = &mut *(context as *mut IrLoweringA64);
        self_.regs.restore_reg(&mut *inst);
    }
}
