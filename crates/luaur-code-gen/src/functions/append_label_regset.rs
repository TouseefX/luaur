use crate::functions::append_register_set::append_register_set;
use crate::records::ir_to_string_context::IrToStringContext;
use crate::records::register_set::RegisterSet;

pub fn append_label_regset(
    ctx: &mut IrToStringContext,
    reg_sets: &[RegisterSet],
    block_idx: usize,
    name: &str,
) {
    if block_idx < reg_sets.len() {
        let rs = &reg_sets[block_idx];

        if rs.regs.iter().any(|&r| r != 0) || rs.vararg_seq {
            crate::functions::append::append(&mut ctx.result, format_args!("|{{{}|", name));
            append_register_set(ctx, rs, c"|".as_ptr());
            crate::functions::append::append(&mut ctx.result, format_args!("}}"));
        }
    }
}
