use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl LintLocalHygiene {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        self.locals.get_or_insert(unsafe { (*node).local }).used = true;
        true
    }
}
