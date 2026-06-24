use crate::records::scope::Scope;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn visit_ast_expr_constant_nil(&mut self, expr: *mut AstExprConstantNil) {
        #[cfg(any(not(debug_assertions), feature = "luau_enable_assert"))]
        {
            unsafe {
                let builtin_types = &*self.builtin_types;
                let expected_type = builtin_types.nilType;
                let actual_type =
                    self.lookup_type(expr as *mut luaur_ast::records::ast_expr::AstExpr);
                let scope = self.find_innermost_scope((*expr).base.base.location);

                let subtyping = &mut *self.subtyping;
                let r = subtyping.is_subtype_type_id_type_id_not_null_scope(
                    actual_type,
                    expected_type,
                    scope,
                );

                LUAU_ASSERT!(
                    r.is_subtype
                        || self.is_error_suppressing_location_type_id(
                            (*expr).base.base.location,
                            actual_type
                        )
                );
            }
        }
    }
}
