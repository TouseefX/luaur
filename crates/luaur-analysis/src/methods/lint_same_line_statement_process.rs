use crate::records::lint_context::LintContext;
use crate::records::lint_same_line_statement::LintSameLineStatement;

impl LintSameLineStatement {
    pub fn process(&mut self, context: &mut LintContext) {
        self.context = context as *mut LintContext;
        self.last_line = !0;
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, self);
        }
    }
}
