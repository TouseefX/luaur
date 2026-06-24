use crate::functions::block_index::block_index;

pub fn check_predecessors(
    cfg: &luaur_analysis::records::control_flow_graph::ControlFlowGraph,
    block: *mut luaur_analysis::records::block::Block,
    expected: &[usize],
) {
    let preds = unsafe { (*block).get_predecessors() };
    assert_eq!(expected.len(), preds.len());

    for (pred, expected_index) in preds.iter().zip(expected.iter()) {
        assert_eq!(*expected_index, block_index(cfg, *pred));
    }
}
