extern crate alloc;

use crate::enums::ir_block_kind::IrBlockKind;
use crate::functions::append::append;
use crate::functions::dom_children::dom_children;
use crate::functions::successors::successors;
use crate::functions::to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32 as to_string_block;
use crate::records::ir_function::IrFunction;
use crate::records::ir_to_string_context::ir_to_string_context;
use alloc::string::String;
use core::ffi::c_void;

pub fn to_dot_dj_graph(function: &IrFunction) -> String {
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

        for i in 0..function.blocks.len() {
            let block = function.blocks[i];

            append(&mut ctx.result, format_args!("b{} [", i as u32));

            if block.kind == IrBlockKind::Fallback {
                append(
                    &mut ctx.result,
                    format_args!("style=filled;fillcolor=salmon;"),
                );
            } else if block.kind == IrBlockKind::Bytecode {
                append(
                    &mut ctx.result,
                    format_args!("style=filled;fillcolor=palegreen;"),
                );
            }

            ctx.result.push_str("label=\"");
            to_string_block(&mut ctx, &block, i as u32);
            ctx.result.push_str("\"];\n");
        }

        let cfg = ctx.cfg;

        // Layer by depth in tree
        let mut depth: u32 = 0;
        let mut found = true;

        while found {
            found = false;

            ctx.result.push_str("{rank = same;");
            for i in 0..cfg.dom_ordering.len() {
                if cfg.dom_ordering[i].depth == depth {
                    append(&mut ctx.result, format_args!("b{};", i as u32));
                    found = true;
                }
            }
            ctx.result.push_str("}\n");

            depth += 1;
        }

        for i in 0..cfg.dom_children_offsets.len() {
            let dom = dom_children(cfg, i as u32);

            for target in dom {
                append(
                    &mut ctx.result,
                    format_args!("b{} -> b{};\n", i as u32, target),
                );
            }

            // Join edges are all successor edges that do not strongly dominate
            let succ = successors(cfg, i as u32);

            for successor in succ {
                let mut found_edge = false;

                for target in dom {
                    if target == successor {
                        found_edge = true;
                        break;
                    }
                }

                if !found_edge {
                    append(
                        &mut ctx.result,
                        format_args!("b{} -> b{} [style=dotted];\n", i as u32, successor),
                    );
                }
            }
        }

        ctx.result.push_str("}\n");
    }

    result
}
