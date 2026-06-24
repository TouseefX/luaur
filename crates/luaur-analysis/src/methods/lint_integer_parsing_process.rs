use crate::records::lint_context::LintContext;
use crate::records::lint_integer_parsing::LintIntegerParsing;

impl LintIntegerParsing {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintIntegerParsing {
            context: context as *mut LintContext,
        };
        unsafe {
            luaur_ast::visit::ast_stat_visit(context.root, &mut pass);
        }
    }
}
