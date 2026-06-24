use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_pack_variadic(&mut self, tp: *mut AstTypePackVariadic) {
        unsafe {
            self.visit_ast_type((*tp).variadic_type);
        }
    }
}
