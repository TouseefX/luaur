use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_type_alias(&mut self, t: *mut AstStatTypeAlias) -> ControlFlow {
        unsafe {
            let t = &*t;

            let unreachable = self.make_child_scope(ScopeType::Linear);
            let mut ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

            self.visit_generics(t.generics);
            self.visit_generic_packs(t.generic_packs);
            self.visit_type_ast_type(t.type_ptr);

            ps.drop();
        }

        ControlFlow::None
    }
}
