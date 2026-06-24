use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_type_function(&mut self, f: *mut AstStatTypeFunction) -> ControlFlow {
        unsafe {
            let unreachable = self.make_child_scope(ScopeType::Linear);
            let mut ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

            self.visit_expr_ast_expr_function((*f).body);

            ps.drop();
        }

        ControlFlow::None
    }
}
