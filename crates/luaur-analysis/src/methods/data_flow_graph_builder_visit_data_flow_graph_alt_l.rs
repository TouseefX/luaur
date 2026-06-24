use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

fn returns_or_throws(cf: ControlFlow) -> bool {
    cf == ControlFlow::Returns || cf == ControlFlow::Throws
}

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_for_in(&mut self, f: *mut AstStatForIn) -> ControlFlow {
        unsafe {
            let for_scope: *mut DfgScope = self.make_child_scope(ScopeType::Loop);

            let cf;
            {
                let _ps = PushScope::push_scope(&mut self.scope_stack, for_scope);

                for local in (*f).vars.iter() {
                    let local = *local;
                    if !(*local).annotation.is_null() {
                        self.visit_type_ast_type((*local).annotation);
                    }

                    let def = (*self.def_arena).fresh_cell(
                        Symbol::from_local(local),
                        (*local).location,
                        false,
                    );
                    *self.graph.local_defs.get_or_insert(local as *const _) = def;
                    *(*self.current_scope())
                        .bindings
                        .get_or_insert(Symbol::from_local(local)) = def;
                    self.captures
                        .get_or_insert(Symbol::from_local(local))
                        .all_versions
                        .push(def);
                }

                for expr in (*f).values.iter() {
                    self.visit_expr_ast_expr(*expr);
                }

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
