use crate::records::lint_context::LintContext;
use crate::records::lint_unreachable_code::LintUnreachableCode;

impl LintUnreachableCode {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintUnreachableCode {
            context: context as *mut LintContext,
        };
        pass.analyze(context.root);
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
        }
    }
}
