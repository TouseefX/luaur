use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;

pub fn is_identifier(node: *mut AstNode) -> bool {
    if node.is_null() {
        return false;
    }
    let node = unsafe { &*node };
    node.is::<AstExprGlobal>() || node.is::<AstExprLocal>()
}
