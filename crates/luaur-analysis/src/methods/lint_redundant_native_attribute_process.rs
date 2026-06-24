use crate::records::lint_context::LintContext;
use crate::records::lint_redundant_native_attribute::LintRedundantNativeAttribute;

pub fn lint_redundant_native_attribute_process(context: &mut LintContext) {
    let mut pass = LintRedundantNativeAttribute {
        context: context as *mut LintContext,
    };
    unsafe {
        luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
    }
}
