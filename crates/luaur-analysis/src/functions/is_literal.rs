use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_table::AstExprTable;

pub fn is_literal(expr: *const AstExpr) -> bool {
    if expr.is_null() {
        return false;
    }

    let expr = expr as *mut AstExpr;

    if luaur_ast::rtti::ast_node_is::<AstExprTable>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }
    if luaur_ast::rtti::ast_node_is::<AstExprFunction>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }
    if luaur_ast::rtti::ast_node_is::<AstExprConstantNumber>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }
    if luaur_ast::rtti::ast_node_is::<AstExprConstantString>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }
    if luaur_ast::rtti::ast_node_is::<AstExprConstantBool>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }
    if luaur_ast::rtti::ast_node_is::<AstExprConstantNil>(unsafe {
        &*(expr as *mut luaur_ast::records::ast_node::AstNode)
    }) {
        return true;
    }

    false
}
