//! Source: `Analysis/include/Luau/ControlFlowGraph.h:355-358` (hand-ported)
//! C++ `CFGBuilder::BlockScope::~BlockScope()`.
use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
    /// RAII exit. C++ dtor restores the saved block:
    ///   `~BlockScope() { builder.currentBlock = NotNull{saved}; }`
    /// The saved block is not threadable through this nullary signature, so the
    /// restore is performed inline by the lowering scope that owns `saved`
    /// (`lower_ast_stat_if` / `lower_ast_stat_while`). Nothing remains for the
    /// destructor to do here.
    pub fn cfg_builder_block_scope_block_scope(&mut self) {}
}
