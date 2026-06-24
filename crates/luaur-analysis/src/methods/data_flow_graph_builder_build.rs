use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def_arena::DefArena;
use crate::records::dfg_scope::DfgScope;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::push_scope::PushScope;
use crate::records::refinement_key_arena::RefinementKeyArena;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;
use luaur_common::FFlag;

impl DataFlowGraphBuilder {
    pub fn build(
        block: *mut AstStatBlock,
        def_arena: *mut DefArena,
        key_arena: *mut RefinementKeyArena,
        handle: *mut InternalErrorReporter,
    ) -> DataFlowGraph {
        LUAU_TIMETRACE_SCOPE!("DataFlowGraphBuilder::build", "Typechecking");

        let mut builder = DataFlowGraphBuilder::data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(def_arena, key_arena);
        builder.handle = handle;

        let module_scope = {
            let mut scope = Box::new(DfgScope {
                parent: core::ptr::null_mut(),
                scope_type: ScopeType::Linear,
                bindings: crate::type_aliases::bindings::Bindings::new(
                    crate::records::symbol::Symbol::default(),
                ),
                props: crate::type_aliases::props_data_flow_graph::Props::new(core::ptr::null()),
            });
            let ptr = scope.as_mut() as *mut DfgScope;
            builder.scopes.push(scope);
            ptr
        };

        let _ps = PushScope::push_scope(&mut builder.scope_stack, module_scope);
        let _ = builder.visit_block_without_child_scope(block);
        builder.resolve_captures();

        if FFlag::DebugLuauFreezeArena.get() {
            unsafe {
                (*builder.def_arena).allocator.freeze();
                (*builder.key_arena).allocator.freeze();
            }
        }

        // C++: `return std::move(builder.graph);` — move the graph out of the
        // owned builder (the rest of `builder` is dropped here).
        builder.graph
    }
}
