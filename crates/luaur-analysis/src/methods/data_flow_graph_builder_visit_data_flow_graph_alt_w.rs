use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_error::AstStatError;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_error(&mut self, error: *mut AstStatError) -> ControlFlow {
        unsafe {
            let error = &*error;

            let unreachable: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
            let mut ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

            let mut i = 0usize;
            while i < error.statements.size {
                let s = *error.statements.data.add(i);
                self.visit_ast_stat(s);
                i += 1;
            }

            let mut i = 0usize;
            while i < error.expressions.size {
                let e = *error.expressions.data.add(i);
                self.visit_expr_ast_expr(e as *mut AstExpr);
                i += 1;
            }

            ps.drop();
        }

        ControlFlow::None
    }
}
