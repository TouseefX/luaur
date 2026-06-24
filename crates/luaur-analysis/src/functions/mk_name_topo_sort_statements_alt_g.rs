use crate::functions::mk_name_topo_sort_statements::mk_name_ast_local;
use crate::functions::mk_name_topo_sort_statements_alt_b::mk_name_ast_expr_local;
use crate::functions::mk_name_topo_sort_statements_alt_c::mk_name_ast_expr_global;
use crate::functions::mk_name_topo_sort_statements_alt_e::mk_name_ast_expr_index_name;
use crate::functions::mk_name_topo_sort_statements_alt_f::mk_name_ast_expr_error;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::rtti::ast_node_as;

pub fn mk_name_ast_expr(expr: &AstExpr) -> Option<Identifier> {
    unsafe {
        let local = ast_node_as::<AstExprLocal>(
            expr as *const AstExpr as *mut luaur_ast::records::ast_node::AstNode,
        );
        if !local.is_null() {
            return Some(mk_name_ast_expr_local(&*local));
        }

        let global = ast_node_as::<AstExprGlobal>(
            expr as *const AstExpr as *mut luaur_ast::records::ast_node::AstNode,
        );
        if !global.is_null() {
            return Some(mk_name_ast_expr_global(&*global));
        }

        let index_name = ast_node_as::<AstExprIndexName>(
            expr as *const AstExpr as *mut luaur_ast::records::ast_node::AstNode,
        );
        if !index_name.is_null() {
            return mk_name_ast_expr_index_name(&*index_name);
        }

        let error = ast_node_as::<AstExprError>(
            expr as *const AstExpr as *mut luaur_ast::records::ast_node::AstNode,
        );
        if !error.is_null() {
            return Some(mk_name_ast_expr_error(&*error));
        }

        None
    }
}
