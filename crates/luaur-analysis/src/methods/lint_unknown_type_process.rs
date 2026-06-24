use crate::records::lint_context::LintContext;
use crate::records::lint_unknown_type::LintUnknownType;
pub fn lint_unknown_type_process(context: &mut LintContext) {
    let mut pass = LintUnknownType {
        context: context as *mut LintContext,
    };
    unsafe { luaur_ast::visit::ast_stat_visit(context.root, &mut pass) };
}
