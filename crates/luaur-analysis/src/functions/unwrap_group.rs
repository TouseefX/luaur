use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_group::AstExprGroup;

pub fn unwrap_group(mut expr: *mut AstExpr) -> *mut AstExpr {
    while !expr.is_null() {
        let group = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprGroup>(
                expr as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        if group.is_null() {
            break;
        }
        expr = unsafe { (*group).expr };
    }

    expr
}
