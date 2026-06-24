use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl FindExprOrLocal {
    pub fn visit_ast_stat_for_in(&mut self, for_in: *mut AstStatForIn) -> bool {
        unsafe {
            for i in 0..(*for_in).vars.size {
                self.visit_ast_local(*((*for_in).vars.data.add(i)));
            }
        }
        true
    }
}
