use crate::enums::value_context::ValueContext;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    // TypeId TypeChecker::checkLValue(const ScopePtr& scope, const AstExpr& expr, ValueContext ctx)
    // {
    //     return checkLValueBinding(scope, expr, ctx);
    // }
    pub fn check_l_value(&mut self, scope: &ScopePtr, expr: &AstExpr, ctx: ValueContext) -> TypeId {
        // checkLValueBinding(scope, expr, ctx) — the multi-dispatch on the concrete AST node.
        let node = expr as *const AstExpr as *mut AstExpr as *mut AstNode;

        let local = unsafe { ast_node_as::<AstExprLocal>(node) };
        if !local.is_null() {
            return self.check_l_value_binding_scope_ptr_ast_expr_local(scope, unsafe { &*local });
        }

        let global = unsafe { ast_node_as::<AstExprGlobal>(node) };
        if !global.is_null() {
            return self
                .check_l_value_binding_scope_ptr_ast_expr_global(scope, unsafe { &*global });
        }

        let index_name = unsafe { ast_node_as::<AstExprIndexName>(node) };
        if !index_name.is_null() {
            return self.check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(
                scope,
                unsafe { &*index_name },
                ctx,
            );
        }

        let index_expr = unsafe { ast_node_as::<AstExprIndexExpr>(node) };
        if !index_expr.is_null() {
            return self.check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(
                scope,
                unsafe { &*index_expr },
                ctx,
            );
        }

        let error = unsafe { ast_node_as::<AstExprError>(node) };
        if !error.is_null() {
            let expressions = unsafe { (*error).expressions };
            for i in 0..expressions.size as usize {
                let sub_expr = unsafe { *expressions.data.add(i) };
                self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*sub_expr },
                    None,
                    false,
                );
            }
            return self.error_recovery_type_scope_ptr(scope);
        }

        self.ice_string_location("Unexpected AST node in checkLValue", &expr.base.location);
        // ice does not return; produce a value to satisfy the signature.
        self.error_recovery_type_scope_ptr(scope)
    }
}
