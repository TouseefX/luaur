//! C++ `DataFlowGraph::DataFlowGraph(NotNull<DefArena> defArena, NotNull<RefinementKeyArena> keyArena)`
//! (`Analysis/src/DataFlowGraph.cpp:58`). The private arena constructor: stores
//! both arena pointers; every `DenseHashMap` member uses its in-class default
//! initializer with a `nullptr` sentinel key (see `DataFlowGraph.h:60-70`).
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::def_arena::DefArena;
use crate::records::refinement_key_arena::RefinementKeyArena;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl DataFlowGraph {
    pub fn data_flow_graph(
        def_arena: *mut DefArena,
        key_arena: *mut RefinementKeyArena,
    ) -> DataFlowGraph {
        DataFlowGraph {
            def_arena,
            key_arena,
            ast_defs: DenseHashMap::new(std::ptr::null()),
            local_defs: DenseHashMap::new(std::ptr::null()),
            declared_defs: DenseHashMap::new(std::ptr::null()),
            def_to_symbol: DenseHashMap::new(std::ptr::null()),
            ast_refinement_keys: DenseHashMap::new(std::ptr::null()),
        }
    }
}
