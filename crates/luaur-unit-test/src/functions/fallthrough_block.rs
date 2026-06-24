use crate::functions::get_block::get_block;
use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
use luaur_bytecode::records::bc_block::BcBlock;
use luaur_bytecode::type_aliases::bc_edges::BcEdges;
use luaur_bytecode::type_aliases::comp_time_bc_function::CompTimeBcFunction;

pub fn fallthrough_block<'a>(
    fn_: &'a mut CompTimeBcFunction,
    edges: &'a mut BcEdges,
) -> &'a mut BcBlock {
    get_block(fn_, edges, BcBlockEdgeKind::Fallthrough)
}
