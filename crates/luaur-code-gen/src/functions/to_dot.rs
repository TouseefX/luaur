extern crate alloc;

use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::append::append;
use crate::functions::append_blocks::append_blocks;
use crate::records::ir_function::IrFunction;
use crate::records::ir_to_string_context::ir_to_string_context;
use alloc::string::String;
use core::ffi::c_void;

pub fn to_dot(function: &IrFunction, include_inst: bool) -> String {
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
            &mut ctx,
            function,
            include_inst,
            /* include_in */ true,
            /* include_out */ true,
            /* include_def */ true,
        );

        for i in 0..function.blocks.len() {
            let block = function.blocks[i];

            if block.start == !0u32 {
                continue;
            }

            let mut inst_idx = block.start;
            while inst_idx != !0u32 && inst_idx <= block.finish {
                let inst = &function.instructions[inst_idx as usize];

                for op in inst.ops.as_slice() {
                    if op.kind() == IrOpKind::Block {
                        if function.blocks[op.index() as usize].kind != IrBlockKind::Fallback {
                            append(
                                &mut ctx.result,
                                format_args!("b{} -> b{} [weight=10];\n", i as u32, op.index()),
                            );
                        } else {
                            append(
                                &mut ctx.result,
                                format_args!("b{} -> b{};\n", i as u32, op.index()),
                            );
                        }
                    }
                }

                inst_idx += 1;
            }
        }

        ctx.result.push_str("}\n");
    }

    result
}
