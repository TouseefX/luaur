use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def_arena::DefArena;
use crate::records::function_capture::FunctionCapture;
use crate::records::refinement_key_arena::RefinementKeyArena;
use crate::records::symbol::Symbol;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

impl DenseDefault for FunctionCapture {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl DenseDefault for Symbol {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl DataFlowGraphBuilder {
    pub fn data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(
        def_arena: *mut DefArena,
        key_arena: *mut RefinementKeyArena,
    ) -> Self {
        LUAU_ASSERT!(!def_arena.is_null());
        LUAU_ASSERT!(!key_arena.is_null());

        DataFlowGraphBuilder {
            graph: DataFlowGraphBuilder::empty(def_arena, key_arena),
            def_arena,
            key_arena,
            handle: core::ptr::null_mut(),
            scopes: alloc::vec::Vec::new(),
            scope_stack: crate::type_aliases::scope_stack::ScopeStack::new(),
            captures: DenseHashMap::new(Symbol::default()),
        }
    }
}
