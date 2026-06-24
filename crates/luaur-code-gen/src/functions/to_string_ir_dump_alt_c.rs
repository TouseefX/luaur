use crate::functions::get_block_kind_name::get_block_kind_name;
use crate::records::ir_block::IrBlock;
use crate::records::ir_to_string_context::IrToStringContext;

pub fn to_string_ir_to_string_context_ir_block_u32(
    ctx: &mut IrToStringContext,
    block: &IrBlock,
    index: u32,
) {
    crate::functions::append::append(
        &mut ctx.result,
        format_args!("{}_{}", get_block_kind_name(block.kind), index),
    );
}
