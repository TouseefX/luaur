use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_function(&mut self, f: *mut AstExprFunction) -> DataFlowResult {
        let signature_scope = self.make_child_scope(crate::enums::scope_type::ScopeType::Function);
        let _ps = PushScope::push_scope(&mut self.scope_stack, signature_scope);

        self.visit_function(f, signature_scope)
    }
}
