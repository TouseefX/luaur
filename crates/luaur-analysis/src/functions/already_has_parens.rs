use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

pub fn already_has_parens(
    nodes: &luaur_common::records::small_vector::SmallVector<*mut AstNode, 8>,
) -> bool {
    if nodes.is_empty() {
        return false;
    }

    let mut iter = nodes.len();
    while iter > 0 {
        iter -= 1;
        let node = unsafe { *nodes.get_unchecked(iter) };
        let is_valid = unsafe {
            ast_node_is::<AstExprLocal>(&*(node as *mut AstNode))
                || ast_node_is::<AstExprGlobal>(&*(node as *mut AstNode))
                || ast_node_is::<AstExprIndexName>(&*(node as *mut AstNode))
                || ast_node_is::<AstExprIndexExpr>(&*(node as *mut AstNode))
        };
        if !is_valid {
            break;
        }
    }

    if iter == nodes.len() || iter == 0 {
        return false;
    }

    let current_node = unsafe { *nodes.get_unchecked(iter) };
    let call = unsafe { ast_node_as::<AstExprCall>(current_node as *mut AstNode) };
    if call.is_null() {
        return false;
    }

    if iter == 0 {
        return false;
    }

    let prev_node = unsafe { *nodes.get_unchecked(iter - 1) };
    let prev_as_expr = prev_node as *mut luaur_ast::records::ast_expr::AstExpr;
    unsafe { (*call).func == prev_as_expr }
}
