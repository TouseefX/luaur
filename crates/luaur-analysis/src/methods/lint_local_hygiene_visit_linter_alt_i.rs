use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl LintLocalHygiene {
    pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
        let node_ref = unsafe { &*node };
        if !node_ref.self_.is_null() {
            self.locals.get_or_insert(node_ref.self_).arg = true;
        }

        for i in 0..node_ref.args.size {
            let arg = unsafe { *node_ref.args.data.add(i) };
            self.locals.get_or_insert(arg).arg = true;
        }

        true
    }
}
