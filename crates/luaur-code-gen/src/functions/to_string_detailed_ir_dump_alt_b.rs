use crate::enums::include_cfg_info::IncludeCfgInfo;
use crate::enums::include_reg_flow_info::IncludeRegFlowInfo;
use crate::enums::include_use_info::IncludeUseInfo;
use crate::functions::append::append;
use crate::functions::append_block_set::append_block_set;
use crate::functions::append_register_set::append_register_set;
use crate::functions::is_entry_block::is_entry_block;
use crate::functions::pad_to_detail_column::pad_to_detail_column;
use crate::functions::predecessors::predecessors;
use crate::functions::successors::successors;
use crate::functions::to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32 as to_string_block;
use crate::records::ir_block::IrBlock;
use crate::records::ir_to_string_context::IrToStringContext;

pub fn to_string_detailed(
    ctx: &mut IrToStringContext,
    block: &IrBlock,
    block_idx: u32,
    include_use_info: IncludeUseInfo,
    include_cfg_info: IncludeCfgInfo,
    include_reg_flow_info: IncludeRegFlowInfo,
) {
    // Launder the shared CfgInfo reference so register-set reads don't alias the
    // &mut borrow of ctx (the data lives outside ctx; this is sound).
    let cfg = ctx.cfg;

    // Report captured registers for entry block
    if include_reg_flow_info == IncludeRegFlowInfo::Yes
        && is_entry_block(block)
        && cfg.captured.regs.iter().any(|&r| r != 0)
    {
        append(&mut ctx.result, format_args!("; captured regs: "));
        append_register_set(ctx, &cfg.captured, c", ".as_ptr());
        append(&mut ctx.result, format_args!("\n\n"));
    }

    let start = ctx.result.len();

    to_string_block(ctx, block, block_idx);
    append(&mut ctx.result, format_args!(":"));

    if include_use_info == IncludeUseInfo::Yes {
        pad_to_detail_column(&mut ctx.result, start);
        append(
            &mut ctx.result,
            format_args!("; useCount: {}\n", block.use_count),
        );
    } else {
        ctx.result.push_str("\n");
    }

    // Predecessor list
    if include_cfg_info == IncludeCfgInfo::Yes
        && (block_idx as usize) < cfg.predecessors_offsets.len()
    {
        let pred = predecessors(cfg, block_idx);

        if pred.itBegin < pred.itEnd {
            append(&mut ctx.result, format_args!("; predecessors: "));
            append_block_set(ctx, pred);
            append(&mut ctx.result, format_args!("\n"));
        }
    }

    // Successor list
    if include_cfg_info == IncludeCfgInfo::Yes
        && (block_idx as usize) < cfg.successors_offsets.len()
    {
        let succ = successors(cfg, block_idx);

        if succ.itBegin < succ.itEnd {
            append(&mut ctx.result, format_args!("; successors: "));
            append_block_set(ctx, succ);
            append(&mut ctx.result, format_args!("\n"));
        }
    }

    // Live-in VM regs
    if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.r#in.len() {
        let in_rs = cfg.r#in[block_idx as usize];

        if in_rs.regs.iter().any(|&r| r != 0) || in_rs.vararg_seq {
            append(&mut ctx.result, format_args!("; in regs: "));
            append_register_set(ctx, &in_rs, c", ".as_ptr());
            append(&mut ctx.result, format_args!("\n"));
        }
    }

    // Live-out VM regs
    if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.out.len() {
        let out_rs = cfg.out[block_idx as usize];

        if out_rs.regs.iter().any(|&r| r != 0) || out_rs.vararg_seq {
            append(&mut ctx.result, format_args!("; out regs: "));
            append_register_set(ctx, &out_rs, c", ".as_ptr());
            append(&mut ctx.result, format_args!("\n"));
        }
    }
}
