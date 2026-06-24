use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::functions::matches::matches;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_repeat(&mut self, r: *mut AstStatRepeat) -> ControlFlow {
        let repeat_scope = self.make_child_scope(ScopeType::Loop);

        let cf = {
            let _ps = PushScope::push_scope(&mut self.scope_stack, repeat_scope);
            let cf = self.visit_block_without_child_scope(unsafe { (*r).body });
            let _ = self.visit_expr_ast_expr(unsafe { (*r).condition });
            cf
        };

        unsafe {
            (*self.current_scope()).inherit(repeat_scope);
        }

        if matches(cf, ControlFlow::Breaks) || matches(cf, ControlFlow::Continues) {
            ControlFlow::None
        } else {
            cf
        }
    }
}
