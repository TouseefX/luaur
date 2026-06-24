use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_stat_for::AstStatFor;

fn returns_or_throws(cf: ControlFlow) -> bool {
    cf == ControlFlow::Returns || cf == ControlFlow::Throws
}

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_for(&mut self, f: *mut AstStatFor) -> ControlFlow {
        unsafe {
            let for_scope: *mut DfgScope = self.make_child_scope(ScopeType::Loop);

            self.visit_expr_ast_expr((*f).from);
            self.visit_expr_ast_expr((*f).to);
            if !(*f).step.is_null() {
                self.visit_expr_ast_expr((*f).step);
            }

            let cf;
            {
                let _ps = PushScope::push_scope(&mut self.scope_stack, for_scope);

                if !(*(*f).var).annotation.is_null() {
                    self.visit_type_ast_type((*(*f).var).annotation);
                }

                let def = (*self.def_arena).fresh_cell(
                    Symbol::from_local((*f).var),
                    (*(*f).var).location,
                    false,
                );
                *self.graph.local_defs.get_or_insert((*f).var as *const _) = def;
                *(*self.current_scope())
                    .bindings
                    .get_or_insert(Symbol::from_local((*f).var)) = def;
                self.captures
                    .get_or_insert(Symbol::from_local((*f).var))
                    .all_versions
                    .push(def);

                cf = self.visit_ast_stat_block((*f).body);
            }

            let scope = self.current_scope();
            if !returns_or_throws(cf) {
                self.join(scope, scope, for_scope);
            }
        }

        ControlFlow::None
    }
}
