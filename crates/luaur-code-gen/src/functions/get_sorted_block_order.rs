use alloc::vec::Vec;

use crate::enums::ir_block_kind::IrBlockKind;
use crate::functions::get_block_kind_priority::get_block_kind_priority;
use crate::records::ir_function::IrFunction;
use luaur_common::FFlag;

pub fn get_sorted_block_order(function: &mut IrFunction) -> Vec<u32> {
    let mut sorted_blocks: Vec<u32> = Vec::with_capacity(function.blocks.len());
    for i in 0..function.blocks.len() as u32 {
        sorted_blocks.push(i);
    }

    // Native-only helper for sorting blocks. This matches the C++ comparator semantics:
    // - If LuauCodegenVmExitSync is enabled: use get_block_kind_priority ordering first,
    //   and fall back to sortkey/chainkey afterwards.
    // - Otherwise: ensure Fallback blocks are placed at the end, then order by sortkey/chainkey.
    if FFlag::LuauCodegenVmExitSync.get() {
        sorted_blocks.sort_by(|&idx_a, &idx_b| {
            let a = &function.blocks[idx_a as usize];
            let b = &function.blocks[idx_b as usize];

            let pri_a = get_block_kind_priority(a.kind);
            let pri_b = get_block_kind_priority(b.kind);
            if pri_a != pri_b {
                return pri_a.cmp(&pri_b);
            }

            if a.sortkey != b.sortkey {
                return a.sortkey.cmp(&b.sortkey);
            }

            a.chainkey.cmp(&b.chainkey)
        });
    } else {
        sorted_blocks.sort_by(|&idx_a, &idx_b| {
            let a = &function.blocks[idx_a as usize];
            let b = &function.blocks[idx_b as usize];

            let a_is_fallback = a.kind == IrBlockKind::Fallback;
            let b_is_fallback = b.kind == IrBlockKind::Fallback;

            if a_is_fallback != b_is_fallback {
                return (a.kind == IrBlockKind::Fallback).cmp(&(b.kind == IrBlockKind::Fallback));
            }

            if a.sortkey != b.sortkey {
                return a.sortkey.cmp(&b.sortkey);
            }

            a.chainkey.cmp(&b.chainkey)
        });
    }

    sorted_blocks
}
