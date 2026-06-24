use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::assign_index_constraint::AssignIndexConstraint;
use crate::records::assign_prop_constraint::AssignPropConstraint;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::module::Module;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl ConstraintGenerator {
    // ConstraintGenerator::visitLValue(const ScopePtr&, AstExprIndexExpr*, TypeId)
    // (ConstraintGenerator.cpp:3838).
    pub fn visit_l_value_scope_ptr_ast_expr_index_expr_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExprIndexExpr,
        rhs_type: TypeId,
    ) {
        let index = unsafe { (*expr).index };
        let constant_string =
            unsafe { ast_node_as::<AstExprConstantString>(index as *mut AstNode) };
        if !constant_string.is_null() {
            let lhs_ty = self
                .check_scope_ptr_ast_expr(scope, unsafe { (*expr).expr })
                .ty;
            let prop_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };

            if let Some(module) = &self.module {
                let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
                unsafe {
                    *(*module_ptr)
                        .ast_types
                        .get_or_insert(expr as *const AstExpr) = prop_ty;
                    // FIXME? Singleton strings exist.
                    *(*module_ptr)
                        .ast_types
                        .get_or_insert(index as *const AstExpr) = (*self.builtin_types).stringType;
                }
            }

            let prop_name: String = unsafe {
                let value = (*constant_string).value;
                let bytes =
                    core::slice::from_raw_parts(value.data as *const u8, value.size as usize);
                String::from(core::str::from_utf8(bytes).unwrap_or(""))
            };

            let incremented = self.record_property_assignment(lhs_ty);

            let apc = self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                unsafe { (*expr).base.base.location },
                ConstraintV::AssignProp(AssignPropConstraint {
                    lhs_type: lhs_ty,
                    prop_name,
                    rhs_type,
                    prop_location: Some(unsafe { (*index).base.location }),
                    prop_type: prop_ty,
                    decrement_prop_count: incremented,
                }),
            );

            unsafe {
                (*get_mutable_type_id::<BlockedType>(prop_ty)).set_owner(apc as *const _);
            }

            return;
        }

        let lhs_ty = self
            .check_scope_ptr_ast_expr(scope, unsafe { (*expr).expr })
            .ty;
        let index_ty = self.check_scope_ptr_ast_expr(scope, index).ty;
        let prop_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };

        if let Some(module) = &self.module {
            let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
            unsafe {
                *(*module_ptr)
                    .ast_types
                    .get_or_insert(expr as *const AstExpr) = prop_ty;
            }
        }

        let aic = self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            unsafe { (*expr).base.base.location },
            ConstraintV::AssignIndex(AssignIndexConstraint {
                lhs_type: lhs_ty,
                index_type: index_ty,
                rhs_type,
                prop_type: prop_ty,
            }),
        );

        unsafe {
            (*get_mutable_type_id::<BlockedType>(prop_ty)).set_owner(aic as *const _);
        }
    }
}
