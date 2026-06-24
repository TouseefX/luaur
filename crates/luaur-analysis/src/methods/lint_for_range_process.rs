use crate::records::lint_context::LintContext;
use crate::records::lint_for_range::LintForRange;

impl LintForRange {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintForRange {
            context: context as *mut LintContext,
        };
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
        }
    }
}
