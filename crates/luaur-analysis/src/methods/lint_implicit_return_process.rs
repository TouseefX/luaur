use crate::records::lint_context::LintContext;
use crate::records::lint_implicit_return::LintImplicitReturn;

pub fn lint_implicit_return_process(context: &mut LintContext) {
    let mut pass = LintImplicitReturn {
        context: context as *mut LintContext,
    };
    unsafe { luaur_ast::visit::ast_stat_visit(context.root, &mut pass) };
}
