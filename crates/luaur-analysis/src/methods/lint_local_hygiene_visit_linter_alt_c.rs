use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl LintLocalHygiene {
    pub fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
        let info = self.locals.get_or_insert(unsafe { (*node).name });
        info.defined = node.cast();
        info.function = true;

        true
    }
}
