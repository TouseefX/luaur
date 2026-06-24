use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_error::AstStatError;

impl TypeChecker2 {
    pub fn visit_ast_stat_error(&mut self, stat: *mut AstStatError) {
        unsafe {
            let stat = &*stat;
            for i in 0..stat.expressions.size {
                let e = *stat.expressions.data.add(i);
                self.visit_ast_expr_value_context(e, ValueContext::RValue);
            }
            for i in 0..stat.statements.size {
                let s = *stat.statements.data.add(i);
                self.visit_ast_stat(s);
            }
        }
    }
}
