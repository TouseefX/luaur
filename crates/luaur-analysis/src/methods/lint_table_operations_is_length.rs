use crate::records::lint_table_operations::LintTableOperations;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_node::AstNode;

impl LintTableOperations {
    pub fn is_length(&mut self, expr: *mut AstExpr, table: *mut AstExpr) -> bool {
        let n = unsafe { luaur_ast::rtti::ast_node_as::<AstExprUnary>(expr as *mut AstNode) };
        if n.is_null() {
            return false;
        }
        let n_ref = unsafe { &*n };
        n_ref.op == AstExprUnary::Len && crate::functions::similar::similar(n_ref.expr, table)
    }
}
