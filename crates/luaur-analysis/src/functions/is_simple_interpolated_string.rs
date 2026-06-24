use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;

pub fn is_simple_interpolated_string(node: *const AstNode) -> bool {
    if node.is_null() {
        return false;
    }

    let interp_string = unsafe { (*node).as_item::<AstExprInterpString>() };

    if interp_string.is_null() {
        return false;
    }

    unsafe { (*interp_string).expressions.len() == 0 }
}
