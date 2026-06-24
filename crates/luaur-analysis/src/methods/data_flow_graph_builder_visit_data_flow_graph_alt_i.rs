use crate::enums::control_flow::ControlFlow;
use crate::functions::does_call_error::does_call_error;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_stat_expr::AstStatExpr;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_expr(&mut self, e: *mut AstStatExpr) -> ControlFlow {
        unsafe {
            let e = &*e;
            self.visit_expr_ast_expr(e.expr);

            let call = luaur_ast::rtti::ast_node_as::<AstExprCall>(
                e.expr as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !call.is_null() && does_call_error(&*call) {
                ControlFlow::Throws
            } else {
                ControlFlow::None
            }
        }
    }
}
