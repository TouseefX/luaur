use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;

use crate::records::non_strict_context::NonStrictContext;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_varargs(&mut self, _varargs: *mut AstExprVarargs) -> NonStrictContext {
        NonStrictContext::non_strict_context()
    }
}
