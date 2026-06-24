use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_stat_for::AstStatFor;

impl FindExprOrLocal {
    pub fn visit_ast_stat_for(&mut self, for_stat: *mut AstStatFor) -> bool {
        self.visit_ast_local(unsafe { (*for_stat).var });
        true
    }
}
