use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;

impl TypeChecker2 {
    pub fn visit_ast_expr_constant_integer(&mut self, expr: *mut AstExprConstantInteger) {
        unsafe {
            luaur_common::macros::luau_assert::LUAU_ASSERT!(false);

            let _ = expr;
        }
    }
}
