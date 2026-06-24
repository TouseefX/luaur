extern crate alloc;

use crate::functions::block_kind_name::block_kind_name;
use crate::functions::dump_block::dump_block;
use crate::records::control_flow_graph::ControlFlowGraph;
use alloc::string::String;
use core::fmt::Write;

pub fn dump_cfg(cfg: &ControlFlowGraph) -> String {
    let mut result = String::new();
    for i in 0..cfg.blocks.len() {
        let block = cfg.blocks[i];
        unsafe {
            let block = &*block;
            let _ = write!(result, "Block {} ({}", i, block_kind_name(block.kind));
            if !block.debug_name.is_empty() {
                let _ = write!(result, " \"{}\"", block.debug_name);
            }
            result.push(')');

            let successors = block.get_successors();
            if !successors.is_empty() {
                result.push_str(" -> [");
                for j in 0..successors.len() {
                    if j > 0 {
                        result.push_str(", ");
                    }
                    for k in 0..cfg.blocks.len() {
                        if cfg.blocks[k] == successors[j] {
                            let _ = write!(result, "B{}", k);
                            break;
                        }
                    }
                }
                result.push(']');
            }

            result.push_str(":\n");
            result.push_str(&dump_block(block, &cfg.use_defs));
        }
    }
    result
}
