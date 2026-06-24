use crate::enums::alignment_data_x_64::AlignmentDataX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::emit_common_x_64::K_FUNCTION_ALIGNMENT;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use crate::records::lowering_stats::LoweringStats;
use crate::records::module_helpers::ModuleHelpers;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use core::ffi::c_void;

impl IrLoweringX64 {
    pub fn ir_lowering_x_64_ir_lowering_x_64(
        build: &mut AssemblyBuilderX64,
        helpers: &mut ModuleHelpers,
        function: &mut IrFunction,
        stats: *mut LoweringStats,
    ) -> Self {
        let regs = IrRegAllocX64::ir_reg_alloc_x_64_ir_reg_alloc_x_64(build, function, stats);
        let value_tracker = IrValueLocationTracking::new(function);
        let exit_handler_map = luaur_common::records::dense_hash_map::DenseHashMap::new(!0u32);

        let mut self_ = Self {
            build: build as *mut AssemblyBuilderX64,
            helpers: helpers as *mut ModuleHelpers,
            function: function as *mut IrFunction,
            stats,
            regs,
            value_tracker,
            interrupt_handlers: alloc::vec::Vec::new(),
            exit_handlers: alloc::vec::Vec::new(),
            exit_handler_map,
            vector_and_mask: OperandX64::reg(RegisterX64::noreg),
            vector_or_mask: OperandX64::reg(RegisterX64::noreg),
            exit_sync_alloc_token: 0,
            exit_sync_inst_idx: 0,
        };

        build.align(K_FUNCTION_ALIGNMENT, AlignmentDataX64::Ud2);

        self_
    }

    pub fn reset_restore_callback(&mut self) {
        let regs_ptr = &mut self.regs as *mut IrRegAllocX64 as *mut c_void;
        self.value_tracker
            .set_restore_callback(regs_ptr, Some(Self::restore_callback_shim));
    }

    unsafe extern "C" fn restore_callback_shim(context: *mut c_void, inst: *mut IrInst) {
        let regs = &mut *(context as *mut IrRegAllocX64);
        regs.restore(&mut *inst, false);
    }
}
