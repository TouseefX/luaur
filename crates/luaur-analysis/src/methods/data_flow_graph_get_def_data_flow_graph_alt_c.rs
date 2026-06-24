//! C++ `DefId DataFlowGraph::getDef(const AstStatDeclareGlobal* global) const`
//! (`Analysis/src/DataFlowGraph.cpp:86`). Looks the def up in `declaredDefs`,
//! whose keys are `AstStat*`, so the `AstStatDeclareGlobal*` is upcast first.
use crate::records::data_flow_graph::DataFlowGraph;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraph {
    pub fn get_def_for_declare_global(&self, global: *const AstStatDeclareGlobal) -> DefId {
        // C++: auto def = declaredDefs.find(global); LUAU_ASSERT(def); return NotNull{*def};
        let def = self.declared_defs.find(&(global as *const AstStat));
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }
}
