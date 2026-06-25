use crate::functions::get_live_in_out_value_count::get_live_in_out_value_count;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;

pub fn get_live_in_value_count(function: &mut IrFunction, block: &mut IrBlock) -> u32 {
    get_live_in_out_value_count(function, block, false).0
}
