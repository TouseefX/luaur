use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;

impl LintComparisonPrecedence {
    pub fn is_comparison(&self, op: AstExprBinary_Op) -> bool {
        matches!(
            op,
            AstExprBinary_Op::CompareNe
                | AstExprBinary_Op::CompareEq
                | AstExprBinary_Op::CompareLt
                | AstExprBinary_Op::CompareLe
                | AstExprBinary_Op::CompareGt
                | AstExprBinary_Op::CompareGe
        )
    }
}
