use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl LintLocalHygiene {
    pub fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
        let global = self.globals.get_or_insert(unsafe { (*node).name });
        global.used = true;

        if global.firstRef.is_null() {
            global.firstRef = node;
        }

        true
    }
}
