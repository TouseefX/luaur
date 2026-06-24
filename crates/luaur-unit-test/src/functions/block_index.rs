pub fn block_index(
    cfg: &luaur_analysis::records::control_flow_graph::ControlFlowGraph,
    b: *mut luaur_analysis::records::block::Block,
) -> usize {
    for (i, block) in cfg.blocks.iter().enumerate() {
        if *block == b {
            return i;
        }
    }

    panic!("block was not found in CFG");
}
