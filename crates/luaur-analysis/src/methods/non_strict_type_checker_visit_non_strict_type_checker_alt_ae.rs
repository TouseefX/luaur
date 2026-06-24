use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_local_value_context(
        &mut self,
        _local: *mut AstExprLocal,
        _context: ValueContext,
    ) -> NonStrictContext {
        // C++ `visit(AstExprLocal*, ValueContext) { return {}; }`
        NonStrictContext::non_strict_context()
    }
}
