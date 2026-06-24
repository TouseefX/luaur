use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl TypeChecker2 {
    pub fn visit_ast_stat_while(&mut self, while_statement: *mut AstStatWhile) {
        unsafe {
            let while_statement = &*while_statement;
            self.visit_ast_expr_value_context(while_statement.condition, ValueContext::RValue);
            self.visit_ast_stat_block(while_statement.body);
        }
    }
}
