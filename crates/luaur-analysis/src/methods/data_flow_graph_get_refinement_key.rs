use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::refinement_key::RefinementKey;
use luaur_ast::records::ast_expr::AstExpr;

impl DataFlowGraph {
    pub fn get_refinement_key(&self, expr: *const AstExpr) -> *const RefinementKey {
        if let Some(v) = self.ast_refinement_keys.find(&expr) {
            *v
        } else {
            core::ptr::null()
        }
    }
}
