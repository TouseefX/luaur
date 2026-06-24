use crate::records::lint_context::LintContext;
use crate::records::lint_unbalanced_assignment::LintUnbalancedAssignment;

#[inline(never)]
pub fn lint_unbalanced_assignment_process(context: &mut LintContext) {
    let mut pass = LintUnbalancedAssignment {
        context: context as *mut LintContext,
    };
    unsafe { luaur_ast::visit::ast_stat_visit(context.root, &mut pass) };
}
