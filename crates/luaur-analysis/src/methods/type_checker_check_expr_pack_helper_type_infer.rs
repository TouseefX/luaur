use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;

impl TypeChecker {
    pub fn check_expr_pack_helper_scope_ptr_ast_expr(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExpr,
    ) -> WithPredicate<TypePackId> {
        let expr_ptr = expr as *const AstExpr as *mut AstExpr;
        if unsafe { (*expr_ptr).base.class_index == crate::rtti::ast_rtti_index("AstExprCall") } {
            let call_expr = unsafe {
                &*crate::rtti::ast_node_as::<AstExprCall>(
                    expr_ptr as *mut crate::records::ast_node::AstNode,
                )
            };
            return self.check_expr_pack_helper_scope_ptr_ast_expr_call(scope, call_expr);
        } else if unsafe {
            (*expr_ptr).base.class_index == crate::rtti::ast_rtti_index("AstExprVarargs")
        } {
            if scope.vararg_pack.is_none() {
                return WithPredicate::with_predicate_t(
                    self.error_recovery_type_pack_scope_ptr(scope.clone()),
                );
            }
            return WithPredicate::with_predicate_t(scope.vararg_pack.unwrap());
        } else {
            let type_result =
                self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, expr, None, false);
            return WithPredicate::with_predicate_t(self.add_type_pack_type_pack_var(
                TypePackVar::from(TypePack {
                    head: alloc::vec::Vec::from([type_result.r#type]),
                    tail: None,
                }),
            ));
        }
    }
}
