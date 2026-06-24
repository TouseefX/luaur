use crate::records::lint_unused_function::LintUnusedFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl LintUnusedFunction {
    pub fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
        let node_ref = unsafe { &*node };
        let g = self.globals.get_or_insert(node_ref.name);
        g.used = true;
        true
    }
}
