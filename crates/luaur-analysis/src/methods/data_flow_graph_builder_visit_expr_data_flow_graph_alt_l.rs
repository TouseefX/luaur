use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_type_assertion(
        &mut self,
        t: *mut AstExprTypeAssertion,
    ) -> DataFlowResult {
        let expr = unsafe { (*t).expr };
        let annotation = unsafe { (*t).annotation };
        let def = self.visit_expr_ast_expr(expr);
        self.visit_type_ast_type(annotation);
        def
    }
}
