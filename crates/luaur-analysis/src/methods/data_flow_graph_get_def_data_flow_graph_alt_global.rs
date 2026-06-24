//! C++ `DefId DataFlowGraph::getDef(const AstStatDeclareGlobal* global) const`
//! (`Analysis/src/DataFlowGraph.cpp:86`).
use crate::records::data_flow_graph::DataFlowGraph;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraph {
    pub fn get_def_declare_global(&self, global: *const AstStatDeclareGlobal) -> DefId {
        let def = self.declared_defs.find(&(global as *const AstStat));
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }
}
