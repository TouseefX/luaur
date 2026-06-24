use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use luaur_ast::records::ast_expr_group::AstExprGroup;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_group(&mut self, group: *mut AstExprGroup) -> DataFlowResult {
        let expr = unsafe { (*group).expr };
        self.visit_expr_ast_expr(expr)
    }
}
