use crate::records::lint_context::LintContext;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::visit::ast_stat_visit;

impl LintDeprecatedApi {
    #[inline(never)]
    pub fn process(&mut self, context: &mut LintContext) {
        self.lint_deprecated_api(context as *mut LintContext);
        unsafe {
            ast_stat_visit(context.root, self);
        }
    }
}
