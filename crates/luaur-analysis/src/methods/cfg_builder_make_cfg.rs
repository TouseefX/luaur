//! Source: `Analysis/src/ControlFlowGraph.cpp:135-143` (hand-ported)
//! C++ `std::unique_ptr<ControlFlowGraph> CFGBuilder::makeCFG(NotNull<CFGAllocator> allocator, AstStatBlock* block)`.
use crate::records::cfg_allocator::CfgAllocator;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::control_flow_graph::ControlFlowGraph;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::FFlag;

impl CfgBuilder {
    pub fn make_cfg(
        allocator: *mut CfgAllocator,
        block: *mut AstStatBlock,
    ) -> *mut ControlFlowGraph {
        // C++:
        //   CFGBuilder builder(allocator);
        //   builder.lower(block);
        let mut builder = CfgBuilder::new(allocator);
        // `block` is `AstStatBlock*`; C++ `lower(block)` dispatches to the
        // `AstStatBlock*` overload.
        builder.lower_ast_stat_block(block);

        // auto cfg = std::move(builder.cfg);
        let cfg = builder.cfg.take().unwrap();

        // if (FFlag::DebugLuauFreezeArena) allocator->freeze();
        if FFlag::DebugLuauFreezeArena.get() {
            unsafe { (*allocator).freeze() };
        }

        // return cfg;  (unique_ptr -> raw owning pointer)
        alloc::boxed::Box::into_raw(alloc::boxed::Box::new(cfg))
    }
}
