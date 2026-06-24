use crate::functions::append::append;
use crate::functions::to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32;
use crate::records::block_iterator_wrapper::BlockIteratorWrapper;
use crate::records::ir_to_string_context::IrToStringContext;

pub fn append_block_set(ctx: &mut IrToStringContext, blocks: BlockIteratorWrapper) {
    let mut comma = false;

    // Iterate using the wrapper's raw pointer range: [begin, end)
    let mut it = blocks.begin();
    let end = blocks.end();

    while it < end {
        let target = unsafe { *it };
        it = unsafe { it.add(1) };

        if comma {
            append(&mut ctx.result, format_args!(", "));
        }
        comma = true;

        let block: &crate::records::ir_block::IrBlock = &ctx.blocks[target as usize];
        to_string_ir_to_string_context_ir_block_u32(ctx, block, target);
    }
}
