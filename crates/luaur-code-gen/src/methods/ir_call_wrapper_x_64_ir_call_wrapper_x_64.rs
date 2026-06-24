use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;

impl IrCallWrapperX64 {
    pub fn ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        regs: &mut IrRegAllocX64,
        build: &mut AssemblyBuilderX64,
        inst_idx: u32,
    ) -> Self {
        let mut wrapper = Self {
            regs: regs as *mut IrRegAllocX64,
            build: build as *mut AssemblyBuilderX64,
            inst_idx,
            args: [
                crate::records::call_argument::CallArgument::default(),
                crate::records::call_argument::CallArgument::default(),
                crate::records::call_argument::CallArgument::default(),
                crate::records::call_argument::CallArgument::default(),
                crate::records::call_argument::CallArgument::default(),
                crate::records::call_argument::CallArgument::default(),
            ],
            arg_count: 0,
            gpr_pos: 0,
            xmm_pos: 0,
            func_op: crate::records::operand_x_64::OperandX64::operand_x_64_register_x_64(
                crate::records::register_x_64::RegisterX64::noreg,
            ),
            result_reg: crate::records::register_x_64::RegisterX64::noreg,
            result_inst_idx: 0,
            gpr_uses: [0u8; 16],
            xmm_uses: [0u8; 16],
        };

        wrapper.gpr_uses.fill(0);
        wrapper.xmm_uses.fill(0);

        wrapper
    }
}
