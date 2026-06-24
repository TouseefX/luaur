use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::functions::matches::matches;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_while(&mut self, w: *mut AstStatWhile) -> ControlFlow {
        let w = unsafe { &*w };
        let while_scope = self.make_child_scope(ScopeType::Loop);

        let cf = {
            let mut ps = PushScope::push_scope(&mut self.scope_stack, while_scope);
            self.visit_expr_ast_expr(w.condition);
            let cf = self.visit_ast_stat_block(w.body);
            ps.drop();
            cf
        };

        let scope = self.current_scope();
        if !matches(cf, ControlFlow::Returns) && !matches(cf, ControlFlow::Throws) {
            self.join(scope, scope, while_scope);
        }

        ControlFlow::None
    }
}
