use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

impl TypeChecker2 {
    pub fn test_literal_or_ast_type_is_subtype(
        &mut self,
        expr: *mut AstExpr,
        expected_type: TypeId,
    ) -> bool {
        let scope = self.find_innermost_scope(unsafe { (*expr).base.location });
        let expr_ty = self.lookup_type(expr);

        let r = unsafe {
            (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                expr_ty,
                expected_type,
                scope,
            )
        };

        if r.is_subtype {
            return true;
        }

        self.test_potential_literal_is_subtype(expr, expected_type)
    }
}
