use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn visit_l_value_ast_expr_def_id(&mut self, e: *mut AstExpr, incoming_def: DefId) {
        unsafe {
            let node = e as *mut AstNode;
            let def = if (*node).is::<AstExprLocal>() {
                self.visit_l_value_ast_expr_local_def_id(e as *mut AstExprLocal, incoming_def)
            } else if (*node).is::<AstExprGlobal>() {
                self.visit_l_value_ast_expr_global_def_id(e as *mut AstExprGlobal, incoming_def)
            } else if (*node).is::<AstExprIndexName>() {
                self.visit_l_value_ast_expr_index_name_def_id(
                    e as *mut AstExprIndexName,
                    incoming_def,
                )
            } else if (*node).is::<AstExprIndexExpr>() {
                self.visit_l_value_ast_expr_index_expr_def_id(
                    e as *mut AstExprIndexExpr,
                    incoming_def,
                )
            } else if (*node).is::<AstExprError>() {
                self.visit_l_value_ast_expr_error_def_id(e as *mut AstExprError, incoming_def)
            } else {
                LUAU_ASSERT!(false);
                core::ptr::null()
            };

            *self.graph.ast_defs.get_or_insert(e as *const AstExpr) = def as *const Def;
        }
    }
}
