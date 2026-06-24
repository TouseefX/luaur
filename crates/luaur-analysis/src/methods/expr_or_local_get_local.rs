use crate::records::expr_or_local::ExprOrLocal;
use luaur_ast::records::ast_local::AstLocal;

impl ExprOrLocal {
    #[inline]
    pub fn get_local(&self) -> *mut AstLocal {
        self.local
    }
}
