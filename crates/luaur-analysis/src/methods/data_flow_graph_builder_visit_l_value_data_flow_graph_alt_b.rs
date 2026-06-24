use crate::functions::contains_subscripted_definition::contains_subscripted_definition;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl DataFlowGraphBuilder {
    pub fn visit_l_value_ast_expr_local_def_id(
        &mut self,
        l: *mut AstExprLocal,
        incoming_def: DefId,
    ) -> DefId {
        unsafe {
            let scope = self.current_scope();

            if !(*l).upvalue {
                let subscripted = contains_subscripted_definition(incoming_def);
                let symbol = Symbol::from_local((*l).local);
                let updated = (*self.def_arena).fresh_cell(
                    symbol.clone(),
                    (*l).base.base.location,
                    subscripted,
                );
                *(*scope).bindings.get_or_insert(symbol.clone()) = updated;
                self.captures
                    .get_or_insert(symbol)
                    .all_versions
                    .push(updated);
                updated
            } else {
                self.visit_expr_ast_expr(l as *mut AstExpr).def as *const Def
            }
        }
    }
}
