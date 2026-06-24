use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;

impl LintComparisonPrecedence {
    pub fn is_equality(&self, op: AstExprBinary_Op) -> bool {
        matches!(
            op,
            AstExprBinary_Op::CompareNe | AstExprBinary_Op::CompareEq
        )
    }
}
