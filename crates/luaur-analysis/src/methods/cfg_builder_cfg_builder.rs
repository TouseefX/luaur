//! Source: `Analysis/src/ControlFlowGraph.cpp:127-133` (hand-ported)
//! C++ `explicit CFGBuilder::CFGBuilder(NotNull<CFGAllocator> allocator)`.
use crate::enums::block_kind::BlockKind;
use crate::records::cfg_allocator::CfgAllocator;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::control_flow_graph::ControlFlowGraph;
use crate::records::symbol::Symbol;
use alloc::string::ToString;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl CfgBuilder {
    pub fn new(allocator: *mut CfgAllocator) -> Self {
        // C++ member-init order:
        //   cfg(std::make_unique<ControlFlowGraph>(allocator))
        //   allocator(allocator)
        //   currentBlock(cfg->newBlock(BlockKind::Entry, "Entry Block"))
        let mut cfg = ControlFlowGraph::control_flow_graph(allocator);
        let current_block = cfg.new_block(BlockKind::Entry, "Entry Block".to_string());

        let mut builder = Self {
            cfg: Some(cfg),
            allocator,
            current_block,
            // C++ `sealedBlocks{nullptr}`
            sealed_blocks: DenseHashSet::new(core::ptr::null_mut()),
            // C++ `incompleteJoins{nullptr}`
            incomplete_joins: DenseHashMap::new(core::ptr::null_mut()),
            // C++ `versionCounter{Symbol{}}`
            version_counter: DenseHashMap::new(Symbol::default()),
        };

        // C++ constructor body: seal(currentBlock);
        builder.seal(current_block);
        builder
    }
}
