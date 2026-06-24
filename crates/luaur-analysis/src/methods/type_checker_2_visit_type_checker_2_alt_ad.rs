use crate::records::scope::Scope;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn visit_ast_expr_constant_number(&mut self, expr: *mut AstExprConstantNumber) {
        #[cfg(any(not(debug_assertions), feature = "luau_enable_assert"))]
        {
            unsafe {
                let builtin_types = (*self.builtin_types);
                let best_type = builtin_types.number_type;
                let inferred_type =
                    self.lookup_type(expr as *mut luaur_ast::records::ast_expr::AstExpr);
                let scope = self.find_innermost_scope((*expr).base.base.location);

                let subtyping = &*self.subtyping;
                let r = subtyping.is_subtype(best_type, inferred_type, &mut *scope);

                LUAU_ASSERT!(
                    r.is_subtype
                        || self.is_error_suppressing_location_type_id(
                            (*expr).base.base.location,
                            inferred_type
                        )
                );
            }
        }
    }
}
