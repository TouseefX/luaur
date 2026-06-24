use crate::functions::block_index::block_index;

pub fn check_successors(
    cfg: &luaur_analysis::records::control_flow_graph::ControlFlowGraph,
    block: *mut luaur_analysis::records::block::Block,
    expected: &[usize],
) {
    let succs = unsafe { (*block).get_successors() };
    assert_eq!(expected.len(), succs.len());

    for (succ, expected_index) in succs.iter().zip(expected.iter()) {
        assert_eq!(*expected_index, block_index(cfg, *succ));
    }
}
