use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl FindExprOrLocal {
    pub fn visit_ast_stat_local(&mut self, al: *mut AstStatLocal) -> bool {
        unsafe {
            for i in 0..(*al).vars.size {
                self.visit_ast_local(*((*al).vars.data.add(i)));
            }
        }
        true
    }
}
