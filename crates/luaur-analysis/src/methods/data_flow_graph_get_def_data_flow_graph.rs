//! @skeleton-stub
//! C++ `DefId DataFlowGraph::getDef(const AstExpr* expr) const`
//! (`Analysis/src/DataFlowGraph.cpp:64`). Interface-only: the body awaits the
//! full DataFlowGraph port; the signature is the contract its callers depend on.
use crate::records::data_flow_graph::DataFlowGraph;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraph {
    pub fn get_def(&self, expr: *const AstExpr) -> DefId {
        // C++: auto def = astDefs.find(expr); LUAU_ASSERT(def); return NotNull{*def};
        let def = self.ast_defs.find(&expr);
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }
}
