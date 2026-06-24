use luaur_bytecode::enums::bc_block_edge_kind::BcBlockEdgeKind;
use luaur_bytecode::records::bc_op::BcOp;
use luaur_bytecode::type_aliases::bc_edges::BcEdges;

pub fn get_block_op(edges: &BcEdges, kind: BcBlockEdgeKind) -> BcOp {
    for edge in edges.iter() {
        if edge.kind == kind {
            return edge.target;
        }
    }

    panic!("missing bytecode block edge: {:?}", kind)
}
