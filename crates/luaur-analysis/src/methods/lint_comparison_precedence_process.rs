use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
use crate::records::lint_context::LintContext;

impl LintComparisonPrecedence {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintComparisonPrecedence {
            context: context as *mut LintContext,
        };
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
        }
    }
}
