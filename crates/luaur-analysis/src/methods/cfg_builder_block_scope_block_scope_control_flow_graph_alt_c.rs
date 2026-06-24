//! Source: `Analysis/include/Luau/ControlFlowGraph.h:360` (hand-ported)
//! C++ `CFGBuilder::BlockScope::BlockScope(const BlockScope&) = delete;`
use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
    /// Deleted copy constructor in C++ (`= delete`). Mirrors the established
    /// `@delete` convention (e.g. `TypedAllocator` copy ctor): never called.
    #[allow(dead_code)]
    pub fn block_scope_block_scope(&mut self) {
        unimplemented!("BlockScope copy constructor is deleted in C++")
    }
}
