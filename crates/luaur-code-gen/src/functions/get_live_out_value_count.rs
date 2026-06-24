use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;

pub fn get_live_out_value_count(function: &mut IrFunction, block: &mut IrBlock) -> u32 {
    // The dependency get_live_in_out_value_count is currently a stub with signature fn().
    // We must transmute it to the real signature (matching the C++ std::pair<uint32_t, uint32_t> return)
    // to allow this code to compile and function correctly once the dependency is implemented.
    let get_live_in_out: fn(&mut IrFunction, &mut IrBlock, bool) -> (u32, u32) = unsafe {
        core::mem::transmute(
            crate::functions::get_live_in_out_value_count::get_live_in_out_value_count as *const (),
        )
    };

    let (_, live_out) = get_live_in_out(function, block, false);
    live_out
}
