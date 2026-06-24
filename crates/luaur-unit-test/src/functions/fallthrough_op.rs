use crate::functions::get_block_op::get_block_op;
use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
use luaur_bytecode::records::bc_op::BcOp;
use luaur_bytecode::type_aliases::bc_edges::BcEdges;

pub fn fallthrough_op(edges: &BcEdges) -> BcOp {
    get_block_op(edges, BcBlockEdgeKind::Fallthrough)
}
