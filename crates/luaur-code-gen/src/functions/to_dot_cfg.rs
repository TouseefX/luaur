extern crate alloc;

use crate::functions::append::append;
use crate::functions::append_blocks::append_blocks;
use crate::functions::successors::successors;
use crate::records::ir_function::IrFunction;
use crate::records::ir_to_string_context::ir_to_string_context;
use alloc::string::String;
use core::ffi::c_void;

pub fn to_dot_cfg(function: &IrFunction) -> String {
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

        ctx.result.push_str("digraph CFG {\n");
        ctx.result.push_str("node[shape=record]\n");

        append_blocks(
            &mut ctx, function, /* include_inst */ false, /* include_in */ false,
            /* include_out */ false, /* include_def */ true,
        );

        let cfg = ctx.cfg;

        let mut i = 0usize;
        while i < function.blocks.len() && i < cfg.successors_offsets.len() {
            let succ = successors(cfg, i as u32);

            for target in succ {
                append(
                    &mut ctx.result,
                    format_args!("b{} -> b{};\n", i as u32, target),
                );
            }

            i += 1;
        }

        ctx.result.push_str("}\n");
    }

    result
}
