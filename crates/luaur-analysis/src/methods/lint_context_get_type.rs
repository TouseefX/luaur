use crate::records::lint_context::LintContext;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

impl LintContext {
    pub fn get_type(&self, expr: *mut AstExpr) -> Option<TypeId> {
        if self.module.is_null() {
            return None;
        }

        let module = unsafe { &*self.module };
        module.ast_types.find(&(expr as *const AstExpr)).copied()
    }
}
