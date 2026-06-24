//! Source: `Analysis/include/Luau/ControlFlowGraph.h:257` (hand-ported)
//! C++ `struct ControlFlowGraph`.
use crate::records::cfg_allocator::CfgAllocator;
use crate::type_aliases::block_id::BlockId;
use crate::type_aliases::definition::Definition;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    // Maps each use of a local variable (AstExpr*) to the Definition* live at
    // that point. C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
    pub use_defs: DenseHashMap<*mut AstExpr, *mut Definition>,

    pub blocks: Vec<BlockId>,
    pub entry_idx: usize,

    // private:
    pub(crate) allocator: *mut CfgAllocator,
}

unsafe impl Send for ControlFlowGraph {}
unsafe impl Sync for ControlFlowGraph {}
