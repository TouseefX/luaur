//! C++ `LintTableOperations::visit(AstExprUnary*)` (`Analysis/src/Linter.cpp:2610`).

use crate::records::lint_table_operations::LintTableOperations;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_unary::{AstExprUnary, AstExprUnaryOp};

impl LintTableOperations {
    pub fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
        unsafe {
            if (*node).op == AstExprUnaryOp::Len {
                self.check_indexer(node as *mut AstExpr, (*node).expr, "#");
            }
        }

        true
    }
}
