use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_constant_number(
        &mut self,
        _expr: *mut AstExprConstantNumber,
    ) -> NonStrictContext {
        NonStrictContext::non_strict_context()
    }
}
