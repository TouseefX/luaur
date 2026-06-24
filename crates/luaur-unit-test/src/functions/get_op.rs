use luaur_bytecode::records::bc_block::BcBlock;
use luaur_bytecode::records::bc_op::BcOp;

pub fn get_op(block: &BcBlock, idx: usize) -> BcOp {
    *block
        .ops
        .iter()
        .nth(idx)
        .unwrap_or_else(|| panic!("bytecode block op index {idx} out of range"))
}
