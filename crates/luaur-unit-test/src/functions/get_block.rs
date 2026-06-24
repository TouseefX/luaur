use crate::functions::get_block_op::get_block_op;
use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
use luaur_bytecode::records::bc_block::BcBlock;
use luaur_bytecode::type_aliases::bc_edges::BcEdges;
use luaur_bytecode::type_aliases::comp_time_bc_function::CompTimeBcFunction;

pub fn get_block<'a>(
    fn_: &'a mut CompTimeBcFunction,
    edges: &'a mut BcEdges,
    kind: BcBlockEdgeKind,
) -> &'a mut BcBlock {
    let op = get_block_op(edges, kind);
    fn_.block_op(op)
}
