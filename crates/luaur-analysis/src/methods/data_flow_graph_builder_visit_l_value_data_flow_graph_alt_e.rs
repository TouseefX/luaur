use crate::functions::contains_subscripted_definition::contains_subscripted_definition;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def_arena::DefArena;
use crate::records::dfg_scope::DfgScope;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use alloc::string::String;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::location::Location;

impl DataFlowGraphBuilder {
    pub fn visit_l_value_ast_expr_index_expr_def_id(
        &mut self,
        i: *mut AstExprIndexExpr,
        incoming_def: DefId,
    ) -> DefId {
        unsafe {
            let parent_def = self.visit_expr_ast_expr((*i).expr).def as DefId;
            self.visit_expr_ast_expr((*i).index);

            let scope = self.current_scope();
            let index_node = (*i).index as *mut luaur_ast::records::ast_node::AstNode;
            if (*index_node).is::<AstExprConstantString>() {
                let string = &*((*i).index as *mut AstExprConstantString);
                let bytes =
                    core::slice::from_raw_parts(string.value.data as *const u8, string.value.size);
                let key = String::from_utf8_lossy(bytes).into_owned();

                let subscripted = contains_subscripted_definition(incoming_def);
                let updated = (*self.def_arena).fresh_cell(
                    Symbol::default(),
                    (*i).base.base.location,
                    subscripted,
                );
                (*scope)
                    .props
                    .get_or_insert(parent_def)
                    .insert(key, updated);
                return updated;
            } else {
                let subscripted = true;
                (*self.def_arena).fresh_cell(
                    Symbol::default(),
                    (*i).base.base.location,
                    subscripted,
                )
            }
        }
    }
}
