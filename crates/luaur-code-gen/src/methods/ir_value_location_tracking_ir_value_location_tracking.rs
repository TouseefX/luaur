use crate::records::ir_function::IrFunction;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;

impl IrValueLocationTracking {
    pub fn new(function: &mut IrFunction) -> Self {
        let mut tracking = Self {
            function: function as *mut IrFunction,
            vm_reg_value: [crate::records::ir_data::k_invalid_inst_idx; 256],
            vm_reg_dependent: [crate::records::ir_data::k_invalid_inst_idx; 256],
            max_reg: 0,
            restore_callback_ctx: core::ptr::null_mut(),
            restore_callback: None,
        };

        // Mirror the C++ constructor explicitly initializing the arrays.
        tracking
            .vm_reg_value
            .fill(crate::records::ir_data::k_invalid_inst_idx);
        tracking
            .vm_reg_dependent
            .fill(crate::records::ir_data::k_invalid_inst_idx);

        tracking
    }
}
