use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;

impl TypeChecker2 {
    pub fn visit_ast_stat_type_function(&mut self, stat: *mut AstStatTypeFunction) {
        unsafe {
            self.visit_ast_expr_function((*stat).body);
        }
    }
}
