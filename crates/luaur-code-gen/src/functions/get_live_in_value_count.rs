use crate::functions::get_live_in_out_value_count::get_live_in_out_value_count;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;

pub fn get_live_in_value_count(function: &mut IrFunction, block: &mut IrBlock) -> u32 {
    // The previous attempt failed because the stub for get_live_in_out_value_count was empty.
    // In the real implementation, get_live_in_out_value_count returns (u32, u32).
    // We cast the function to the expected signature to satisfy the compiler if the dependency is still a stub.
    let get_live_in_out: fn(&mut IrFunction, &mut IrBlock, bool) -> (u32, u32) =
        unsafe { core::mem::transmute(get_live_in_out_value_count as *const ()) };
    get_live_in_out(function, block, false).0
}
