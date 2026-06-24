use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::append::append;
use crate::functions::get_cmd_name::get_cmd_name;
use crate::functions::has_result::has_result;
use crate::functions::to_string_ir_dump_alt_d::to_string as to_string_op;
use crate::records::ir_inst::IrInst;
use crate::records::ir_to_string_context::IrToStringContext;

pub fn to_string(ctx: &mut IrToStringContext, inst: &IrInst, index: u32) {
    ctx.result.push_str("  ");

    // Instructions with a result display target virtual register
    if has_result(inst.cmd) {
        append(&mut ctx.result, format_args!("%{} = ", index));
    }

    ctx.result.push_str(get_cmd_name(inst.cmd));

    for i in 0..inst.ops.size() as usize {
        let op = inst.ops.as_slice()[i];

        if op.kind() == IrOpKind::None {
            continue;
        }

        ctx.result.push_str(if i == 0 { " " } else { ", " });
        to_string_op(ctx, op);
    }
}
