use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_block(&mut self, b: *mut AstStatBlock) -> ControlFlow {
        LUAU_ASSERT!(!b.is_null());

        let child = self.make_child_scope(ScopeType::Linear);

        let cf;
        {
            let ps = PushScope::push_scope(&mut self.scope_stack, child);
            cf = self.visit_block_without_child_scope(b);
            drop(ps);
        }

        unsafe {
            (*self.current_scope()).inherit(child);
        }
        cf
    }
}
