use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl TypeChecker2 {
    pub fn visit_ast_type_pack_variadic(&mut self, tp: *mut AstTypePackVariadic) {
        unsafe {
            self.visit_ast_type((*tp).variadic_type);
        }
    }
}
