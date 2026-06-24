//! Source: `Analysis/include/Luau/ControlFlowGraph.h:259-262` (hand-ported)
//! C++ `explicit ControlFlowGraph::ControlFlowGraph(NotNull<CFGAllocator> allocator)`.
use crate::records::cfg_allocator::CfgAllocator;
use crate::records::control_flow_graph::ControlFlowGraph;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ControlFlowGraph {
    pub fn control_flow_graph(allocator: *mut CfgAllocator) -> Self {
        Self {
            // C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
            use_defs: DenseHashMap::new(core::ptr::null_mut()),
            // C++ `std::vector<BlockId> blocks;`
            blocks: alloc::vec::Vec::new(),
            // C++ `size_t entryIdx = 0;`
            entry_idx: 0,
            // C++ member-init `: allocator(allocator)`
            allocator,
        }
    }
}
