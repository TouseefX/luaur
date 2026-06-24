use crate::records::block::Block;
use crate::records::cfg_allocator::CfgAllocator;
use crate::records::control_flow_graph::ControlFlowGraph;
use crate::records::join::Join;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use alloc::collections::BTreeSet;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct CfgBuilder {
    pub cfg: Option<ControlFlowGraph>,
    pub allocator: *mut CfgAllocator,
    pub current_block: *mut Block,
    pub sealed_blocks: DenseHashSet<*mut Block>,
    pub incomplete_joins: DenseHashMap<*mut Block, BTreeSet<*mut Join>>,
    pub version_counter: DenseHashMap<Symbol, usize>,
}
