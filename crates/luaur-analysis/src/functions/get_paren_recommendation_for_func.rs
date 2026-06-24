use crate::enums::parentheses_recommendation::ParenthesesRecommendation;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::records::function_type::FunctionType;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

pub fn get_paren_recommendation_for_func(
    func: &FunctionType,
    nodes: &alloc::vec::Vec<*mut AstNode>,
) -> ParenthesesRecommendation {
    if already_has_parens_vec(nodes) {
        return ParenthesesRecommendation::None;
    }

    let last_node = unsafe { *nodes.last().unwrap() };
    let idx_expr = unsafe { ast_node_as::<AstExprIndexName>(last_node as *mut AstNode) };
    let has_implicit_self =
        !idx_expr.is_null() && unsafe { (*idx_expr).op == ':' as core::ffi::c_char };

    let (arg_types, arg_variadic_pack) = flatten_type_pack_id(func.arg_types);

    if let Some(variadic_pack) = arg_variadic_pack {
        if is_variadic(variadic_pack) {
            return ParenthesesRecommendation::CursorInside;
        }
    }

    let no_arg_function = arg_types.is_empty() || (has_implicit_self && arg_types.len() == 1);
    if no_arg_function {
        ParenthesesRecommendation::CursorAfter
    } else {
        ParenthesesRecommendation::CursorInside
    }
}

fn already_has_parens_vec(nodes: &alloc::vec::Vec<*mut AstNode>) -> bool {
    if nodes.is_empty() {
        return false;
    }

    let mut iter = nodes.len();
    while iter > 0 {
        iter -= 1;
        let node = nodes[iter];
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

    let current_node = nodes[iter];
    let call = unsafe { ast_node_as::<AstExprCall>(current_node as *mut AstNode) };
    if call.is_null() {
        return false;
    }

    let prev_node = nodes[iter - 1];
    unsafe { (*call).func == prev_node as *mut luaur_ast::records::ast_expr::AstExpr }
}
