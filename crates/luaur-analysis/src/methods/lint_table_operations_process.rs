use crate::records::lint_context::LintContext;
use crate::records::lint_table_operations::LintTableOperations;

impl LintTableOperations {
    pub fn process(context: &mut LintContext) {
        if context.module.is_null() {
            return;
        }

        let mut pass = LintTableOperations {
            context: context as *mut LintContext,
        };

        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
        }
    }
}
