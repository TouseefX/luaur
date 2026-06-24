extern crate alloc;

use crate::enums::include_cfg_info::IncludeCfgInfo;
use crate::enums::include_reg_flow_info::IncludeRegFlowInfo;
use crate::enums::include_use_info::IncludeUseInfo;
use crate::enums::ir_block_kind::IrBlockKind;
use crate::functions::append::append;
use crate::functions::is_pseudo::is_pseudo;
use crate::functions::to_string_detailed_ir_dump::to_string_detailed as to_string_detailed_inst;
use crate::functions::to_string_detailed_ir_dump_alt_b::to_string_detailed as to_string_detailed_block;
use crate::functions::to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32 as to_string_block;
use crate::records::ir_function::IrFunction;
use crate::records::ir_to_string_context::ir_to_string_context;
use alloc::string::String;
use core::ffi::c_void;

const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;

pub fn to_string(function: &mut IrFunction, include_use_info: IncludeUseInfo) -> String {
    let mut result = String::new();

    {
        let mut ctx = ir_to_string_context {
            result: &mut result,
            blocks: &function.blocks,
            constants: &function.constants,
            cfg: &function.cfg,
            vm_exit_info: &function.vm_exit_info,
            proto: function.proto as *mut c_void,
        };

        for i in 0..function.blocks.len() {
            let block = function.blocks[i];

            if block.kind == IrBlockKind::Dead {
                continue;
            }

            to_string_detailed_block(
                &mut ctx,
                &block,
                i as u32,
                include_use_info,
                IncludeCfgInfo::Yes,
                IncludeRegFlowInfo::Yes,
            );

            if block.start == !0u32 {
                append(&mut ctx.result, format_args!(" *empty*\n\n"));
                continue;
            }

            if (block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
                append(
                    &mut ctx.result,
                    format_args!("   implicit CHECK_SAFE_ENV exit({})\n", block.startpc),
                );
            }

            // To allow dumping blocks that are still being constructed, we can't rely on
            // terminator and need a bounds check
            let mut index = block.start;
            while index <= block.finish && (index as usize) < function.instructions.len() {
                let inst = &mut function.instructions[index as usize];

                // Skip pseudo instructions unless they are still referenced
                if is_pseudo(inst.cmd) && inst.use_count == 0 {
                    index += 1;
                    continue;
                }

                append(&mut ctx.result, format_args!(" "));
                to_string_detailed_inst(&mut ctx, &block, i as u32, inst, index, include_use_info);

                index += 1;
            }

            if block.expected_next_block != !0u32 {
                append(&mut ctx.result, format_args!("; glued to: "));
                let glued = function.blocks[block.expected_next_block as usize];
                to_string_block(&mut ctx, &glued, block.expected_next_block);
                append(&mut ctx.result, format_args!("\n"));
            }

            append(&mut ctx.result, format_args!("\n"));
        }
    }

    result
}
