//! Source: `Analysis/src/ConstraintGenerator.cpp:3550-3570` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprInstantiate* explicitTypeInstantiation)`.
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::records::scope::Scope;
use crate::records::type_instantiation_constraint::TypeInstantiationConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_instantiate(
        &mut self,
        scope: &ScopePtr,
        explicit_type_instantiation: *mut AstExprInstantiate,
    ) -> Inference {
        unsafe {
            if !luaur_common::FFlag::LuauExplicitTypeInstantiationSupport.get() {
                return self.check_scope_ptr_ast_expr(scope, (*explicit_type_instantiation).expr);
            }

            let function_type = self
                .check_scope_ptr_ast_expr_optional_type_id(
                    scope,
                    (*explicit_type_instantiation).expr,
                    None,
                )
                .ty;

            let (explicit_type_ids, explicit_type_pack_ids) = self.resolve_type_arguments(
                scope.as_ref() as *const Scope as *mut Scope,
                (*explicit_type_instantiation).type_arguments,
            );

            let placeholder_type = (*self.arena).add_type(BlockedType::default());

            let constraint = self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                (*explicit_type_instantiation).base.base.location,
                ConstraintV::TypeInstantiation(TypeInstantiationConstraint {
                    function_type,
                    placeholder_type,
                    type_arguments: explicit_type_ids,
                    type_pack_arguments: explicit_type_pack_ids,
                }),
            );

            let blocked = get_mutable_type_id::<BlockedType>(placeholder_type);
            (*blocked).set_owner(constraint as *const _);

            Inference::inference_type_id_refinement_id(placeholder_type, core::ptr::null_mut())
        }
    }
}
