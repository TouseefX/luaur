use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl TypeChecker2 {
    pub fn visit_ast_type_typeof(&mut self, ty: *mut AstTypeTypeof) {
        unsafe {
            self.visit_ast_expr_value_context((*ty).expr, ValueContext::RValue);
        }
    }
}
