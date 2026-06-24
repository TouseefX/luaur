use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl ConstraintGenerator {
    // ConstraintGenerator::visitLValue(const ScopePtr&, AstExpr*, TypeId)
    // (ConstraintGenerator.cpp:3733). The C++ overload-on-static-type dispatch
    // is recovered here with RTTI.
    pub fn visit_l_value_scope_ptr_ast_expr_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExpr,
        rhs_type: TypeId,
    ) {
        let node = expr as *mut AstNode;

        let e_local = unsafe { ast_node_as::<AstExprLocal>(node) };
        if !e_local.is_null() {
            self.visit_l_value_scope_ptr_ast_expr_local_type_id(scope, e_local, rhs_type);
            return;
        }

        let e_global = unsafe { ast_node_as::<AstExprGlobal>(node) };
        if !e_global.is_null() {
            self.visit_l_value_scope_ptr_ast_expr_global_type_id(scope, e_global, rhs_type);
            return;
        }

        let e_index_name = unsafe { ast_node_as::<AstExprIndexName>(node) };
        if !e_index_name.is_null() {
            self.visit_l_value_scope_ptr_ast_expr_index_name_type_id(scope, e_index_name, rhs_type);
            return;
        }

        let e_index_expr = unsafe { ast_node_as::<AstExprIndexExpr>(node) };
        if !e_index_expr.is_null() {
            self.visit_l_value_scope_ptr_ast_expr_index_expr_type_id(scope, e_index_expr, rhs_type);
            return;
        }

        let e_error = unsafe { ast_node_as::<AstExprError>(node) };
        if !e_error.is_null() {
            // If we end up with some sort of error expression in an lvalue
            // position, at least go and check the expressions so that when
            // we visit them later, there aren't any invalid assumptions.
            let expressions = unsafe { (*e_error).expressions };
            for i in 0..expressions.size as usize {
                let sub_expr = unsafe { *expressions.data.add(i) };
                self.check_scope_ptr_ast_expr(scope, sub_expr);
            }
            return;
        }

        unsafe {
            (*self.ice).ice_string_location("Unexpected lvalue expression", &(*expr).base.location);
        }
    }
}
