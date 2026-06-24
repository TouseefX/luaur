use crate::records::lint_unused_function::LintUnusedFunction;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl LintUnusedFunction {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        unsafe {
            let name = (*node).name;
            let expr = luaur_ast::rtti::ast_node_as::<AstExprGlobal>(name as *mut AstNode);
            if !expr.is_null() {
                let g = self.globals.get_or_insert((*expr).name);
                g.function = true;
                g.location = (*expr).base.base.location;

                luaur_ast::visit::ast_expr_visit((*node).func as *mut AstExpr, self);

                return false;
            }
        }

        true
    }
}
