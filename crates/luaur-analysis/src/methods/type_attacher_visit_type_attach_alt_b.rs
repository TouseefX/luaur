use crate::records::type_attacher::TypeAttacher;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl TypeAttacher {
    pub fn visit_ast_expr_local(&mut self, al: *mut AstExprLocal) -> bool {
        let al_ref = unsafe { &*al };
        self.visit_local(al_ref.local)
    }
}
