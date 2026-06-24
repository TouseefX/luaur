use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::rtti::ast_node_as;

impl LintComparisonPrecedence {
    pub fn is_not(&self, node: *mut AstExpr) -> bool {
        let expr = unsafe {
            ast_node_as::<AstExprUnary>(node as *mut luaur_ast::records::ast_node::AstNode)
        };
        !expr.is_null()
            && unsafe { (*expr).op == luaur_ast::records::ast_expr_unary::AstExprUnaryOp::Not }
    }
}
