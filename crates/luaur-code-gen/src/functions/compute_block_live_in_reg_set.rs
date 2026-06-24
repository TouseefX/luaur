use crate::functions::visit_vm_reg_defs_uses_ir_visit_use_def_alt_b::visit_vm_reg_defs_uses_t_ir_function_ir_block;
use crate::records::block_vm_reg_live_in_computation::BlockVmRegLiveInComputation;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::register_set::RegisterSet;

pub fn compute_block_live_in_reg_set(
    function: &mut IrFunction,
    block: &IrBlock,
    def_rs: &mut RegisterSet,
    captured_regs: &mut [u64; 4],
) -> RegisterSet {
    let mut visitor = BlockVmRegLiveInComputation::new(def_rs, captured_regs);
    visit_vm_reg_defs_uses_t_ir_function_ir_block(&mut visitor, function, block);
    visitor.in_rs
}
