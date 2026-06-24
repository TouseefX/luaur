use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_declare_extern_type(
        &mut self,
        d: *mut AstStatDeclareExternType,
    ) -> ControlFlow {
        unsafe {
            let d = &*d;

            // This declaration does not "introduce" any bindings in value namespace,
            // so there's no symbolic value to begin with. We'll traverse the properties
            // because their type annotations may depend on something in the value namespace.
            let unreachable = self.make_child_scope(crate::enums::scope_type::ScopeType::Linear);
            let mut _ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

            for i in 0..d.props.size {
                let prop = *d.props.data.add(i);
                self.visit_type_ast_type(prop.ty);
            }
        }

        ControlFlow::None
    }
}
