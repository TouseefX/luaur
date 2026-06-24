use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl FindExprOrLocal {
    pub fn visit_ast_stat_local_function(&mut self, function: *mut AstStatLocalFunction) -> bool {
        self.visit_ast_local(unsafe { (*function).name });
        true
    }
}
