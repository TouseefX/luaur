use crate::enums::value_context::ValueContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_typeof(&mut self, type_of: *mut AstTypeTypeof) {
        unsafe {
            self.visit_ast_expr_value_context((*type_of).expr, ValueContext::RValue);
        }
    }
}
