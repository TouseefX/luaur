use crate::records::lint_context::LintContext;
use crate::records::lint_misleading_and_or::LintMisleadingAndOr;

impl LintMisleadingAndOr {
    pub fn process(&mut self, context: &mut LintContext) {
        self.context = context as *mut LintContext;
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, self);
        }
    }
}
