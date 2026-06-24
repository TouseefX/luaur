use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::assign_prop_constraint::AssignPropConstraint;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::module::Module;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use core::ffi::CStr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl ConstraintGenerator {
    // ConstraintGenerator::visitLValue(const ScopePtr&, AstExprIndexName*, TypeId)
    // (ConstraintGenerator.cpp:3825).
    pub fn visit_l_value_scope_ptr_ast_expr_index_name_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExprIndexName,
        rhs_type: TypeId,
    ) {
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
            }
        }

        let incremented = self.record_property_assignment(lhs_ty);

        let prop_name: String = unsafe {
            CStr::from_ptr((*expr).index.value)
                .to_string_lossy()
                .into_owned()
        };

        let apc = self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            unsafe { (*expr).base.base.location },
            ConstraintV::AssignProp(AssignPropConstraint {
                lhs_type: lhs_ty,
                prop_name,
                rhs_type,
                prop_location: Some(unsafe { (*expr).index_location }),
                prop_type: prop_ty,
                decrement_prop_count: incremented,
            }),
        );

        unsafe {
            (*get_mutable_type_id::<BlockedType>(prop_ty)).set_owner(apc as *const _);
        }
    }
}
