use crate::records::expr_or_local::ExprOrLocal;
use luaur_ast::records::ast_local::AstLocal;

impl ExprOrLocal {
    pub fn set_local(&mut self, new_local: *mut AstLocal) {
        self.local = new_local;
        self.expr = core::ptr::null_mut();
    }
}
