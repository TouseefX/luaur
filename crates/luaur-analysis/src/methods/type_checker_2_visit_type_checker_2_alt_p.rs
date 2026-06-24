use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl TypeChecker2 {
    pub fn visit_ast_stat_local_function(&mut self, stat: *mut AstStatLocalFunction) {
        unsafe {
            let func = (*stat).func;
            self.visit_ast_expr_function(func);
        }
    }
}
