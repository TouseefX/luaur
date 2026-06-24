use crate::functions::kill_ir_utils::kill_ir_function_ir_inst;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;

pub fn remove_inst_use(function: &mut IrFunction, inst_idx: u32) {
    let inst: *mut IrInst = &mut function.instructions[inst_idx as usize];

    CODEGEN_ASSERT!(unsafe { (*inst).use_count } != 0);
    unsafe {
        (*inst).use_count -= 1;
    }

    if unsafe { (*inst).use_count } == 0 {
        kill_ir_function_ir_inst(function, unsafe { &mut *inst });
    }
}
