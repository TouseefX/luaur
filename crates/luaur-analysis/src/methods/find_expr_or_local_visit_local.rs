use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_local::AstLocal;

impl FindExprOrLocal {
    pub fn visit_ast_local(&mut self, local: *mut AstLocal) -> bool {
        if self.is_closer_match(unsafe { (*local).location }) {
            self.result.set_local(local);
            return true;
        }
        false
    }
}
