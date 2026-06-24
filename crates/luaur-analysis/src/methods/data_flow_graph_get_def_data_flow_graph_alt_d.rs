//! C++ `DefId DataFlowGraph::getDef(const AstStatDeclareFunction* func) const`
//! (`Analysis/src/DataFlowGraph.cpp:93`). Looks the def up in `declaredDefs`,
//! whose keys are `AstStat*`, so the `AstStatDeclareFunction*` is upcast first.
use crate::records::data_flow_graph::DataFlowGraph;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraph {
    pub fn get_def_for_declare_function(&self, func: *const AstStatDeclareFunction) -> DefId {
        // C++: auto def = declaredDefs.find(func); LUAU_ASSERT(def); return NotNull{*def};
        let def = self.declared_defs.find(&(func as *const AstStat));
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }
}
